/// These are macros to create the 3 different set read functions for a specific BED struct
///
/// 1. read_{}_set
/// 2. read_{}_set_with
/// 3. read_into_{}_set_with
///
/// The first function assumes no set or translater is initialized and does full initialization
/// The second function assumes a translater is initialized and does not initialize a new one
/// The third function assumes a set and translater are initialized and reads into the set
#[macro_export]
macro_rules! create_set_io {
    ($name:ident, $set_type:ty, $num_ty:ty) => {
        paste::item! {

            /// `read_into_{}_set_with` reads into an existing set using a reader and translater
            pub fn [<read_into_ $name _set_with>]<R: Read>(
                reader: R,
                set: &mut $set_type,
                translater: Option<&mut SplitTranslater>,
            ) -> Result<()> {
                if let Some(translater) = translater {
                    [<read_ $name _set_named>](reader, set, translater)
                } else {
                    [<read_ $name _set_numeric>](reader, set)
                }
            }

            /// `read_{}_set_with` reads a set with a translater. Initializes a set then
            /// passes it to `read_into_{}_set_with`
            pub fn [<read_ $name _set_with>]<R: Read>(
                reader: R,
                translater: Option<&mut SplitTranslater>,
            ) -> Result<$set_type> {
                let mut set = $set_type::empty();
                [<read_into_ $name _set_with>](reader, &mut set, translater)?;
                Ok(set)
            }

            /// `read_{}_set` initializes both a new set and translater and passes them to
            /// `read_into_{}_set_with`
            pub fn [<read_ $name _set>]<R: Read>(
                reader: R,
                named: bool,
            ) -> Result<($set_type, Option<SplitTranslater>)> {
                let mut set = $set_type::empty();
                let mut translater = if named {
                    Some(SplitTranslater::new())
                } else {
                    None
                };
                [<read_into_ $name _set_with>](reader, &mut set, translater.as_mut())?;
                Ok((set, translater))
            }

            /// A macro-generated deserialization for fully numeric inputs
            pub fn [<read_ $name _set_numeric>]<R: Read>(reader: R, set: &mut $set_type) -> Result<()> {
                let mut reader = build_reader(reader);
                for record in reader.deserialize() {
                    let record: $num_ty = record?;
                    set.insert(record);
                }
                Ok(())
            }
        }
    };
}

// These are macros to generate the set functions for the BedReader struct
// and return the appropriate type with the associated functions
#[macro_export]
macro_rules! create_set_fn {
    ($func_name:ident, $set_type:ty, $read_func:ident) => {
        pub fn $func_name(self) -> Result<($set_type, Option<SplitTranslater>)> {
            let is_named = self.is_named();
            $read_func(self.reader(), is_named)
        }
    };
}

#[macro_export]
macro_rules! create_io {
    ($name:ident, $type:ty) => {
        paste::item! {
            /// Generates a function for reading a set without additional parameters
            pub fn [<$name _set>](self) -> Result<($type, Option<SplitTranslater>)> {
                let is_named = self.is_named();
                [<read_ $name _set>](self.reader(), is_named)
            }

            /// Generates a function for reading a set with additional parameters
            pub fn [<$name _set_with>](self, translator: Option<&mut SplitTranslater>) -> Result<$type> {
                [<read_ $name _set_with>](self.reader(), translator)
            }

            /// Generates a function for reading into an existing set with additional parameters
            pub fn [<into_ $name _set_with>](self, set: &mut $type, translater: Option<&mut SplitTranslater>) -> Result<()> {
                [<read_into_ $name _set_with>](self.reader(), set, translater)
            }
        }
    };
}
