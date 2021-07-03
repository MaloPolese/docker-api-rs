macro_rules! impl_vec_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], S>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: IntoIterator<Item = S>,
                S: AsRef<str> + serde::Serialize
            {
                self.params.insert($docker_name, serde_json::json!($name.into_iter().collect::<Vec<_>>()));
                self
            }
        }
    };
}

macro_rules! impl_field {
    ($($docs:literal)* $name:ident: $ty:ty => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](&mut self, $name: $ty)-> &mut Self
            {
                self.params.insert($docker_name, serde_json::json!($name));
                self
            }
        }
    };
}

macro_rules! impl_str_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<[< $ty >]>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: AsRef<str> + serde::Serialize,
            {
                self.params.insert($docker_name, serde_json::json!($name.as_ref()));
                self
            }
        }
    };
}

macro_rules! impl_url_str_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<[< $ty >]>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: Into<String> + serde::Serialize,
            {
                self.params.insert($docker_name, $name.into());
                self
            }
        }
    };
}

macro_rules! impl_url_bool_field {
    ($($docs:literal)* $name:ident => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](&mut self, $name: bool)-> &mut Self {
                self.params.insert($docker_name, $name.to_string());
                self
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_str_enum_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](&mut self, $name: $ty)-> &mut Self
            {
                self.params.insert($docker_name, serde_json::json!($name.as_ref()));
                self
            }
        }
    };
}

macro_rules! impl_map_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], K, V>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: IntoIterator<Item = (K, V)>,
                K: AsRef<str> + serde::Serialize + Eq + std::hash::Hash,
                V: AsRef<str> + serde::Serialize
            {
                self.params.insert($docker_name, serde_json::json!($name.into_iter().collect::<std::collections::HashMap<_, _>>()));
                self
            }
        }
    };
}

macro_rules! impl_json_opts_builder {
    ($($docs:literal)* $name:ident) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(serde::Serialize, Debug)]
            pub struct [< $name Opts >] {
                params: std::collections::HashMap<&'static str, serde_json::Value>,
            }

            impl [< $name Opts >] {
                calculated_doc!{
                #[doc = concat!("Returns a new instance of a builder for ", stringify!($name), "Opts.")]
                pub fn builder() -> [< $name OptsBuilder >] {
                    [< $name OptsBuilder >]::default()
                }
                }

                /// Serialize options as a JSON String. Returns None if no options are defined.
                pub fn serialize(&self) -> crate::Result<String> {
                    serde_json::to_string(&self.params).map_err(crate::Error::from)
                }
            }

            calculated_doc!{
            #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
            #[derive(Default, Debug)]
            pub struct [< $name OptsBuilder >] {
                params: std::collections::HashMap<&'static str, serde_json::Value>,
            }
            }

            impl [< $name OptsBuilder >] {
                calculated_doc!{
                #[doc = concat!("Finish building ", stringify!($name), "Opts.")]
                pub fn build(&self) -> [< $name Opts >] {
                    [< $name Opts >] {
                        params: self.params.clone(),
                    }
                }
                }
            }
        }
    };
}

macro_rules! impl_url_opts_builder {
    ($($docs:literal)* $name:ident) => {
        impl_url_opts_builder!{derives = | $($docs)* $name}
    };
    (derives = $($derives:ident),* | $($docs:literal)* $name:ident) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(serde::Serialize, Debug $(,$derives)*)]
            pub struct [< $name Opts >] {
                params: std::collections::HashMap<&'static str, String>
            }

            impl [< $name  Opts >] {
                calculated_doc!{
                #[doc = concat!("Returns a new instance of a builder for ", stringify!($name), "Opts.")]
                pub fn builder() -> [< $name  OptsBuilder >] {
                  [< $name  OptsBuilder >]::default()
                }
                }

                /// Serialize options as a URL query String. Returns None if no options are defined.
                pub fn serialize(&self) -> Option<String> {
                    if self.params.is_empty() {
                        None
                    } else {
                        Some(
                            crate::util::url::encoded_pairs(&self.params)
                        )
                    }
                }
            }

            calculated_doc!{
            #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
            #[derive(Default, Debug)]
            pub struct [< $name  OptsBuilder >] {
                params: std::collections::HashMap<&'static str, String>
            }
            }

            impl [< $name  OptsBuilder >] {
                calculated_doc!{
                #[doc = concat!("Finish building ", stringify!($name), "Opts.")]
                pub fn build(&self) -> [< $name  Opts >] {
                    [< $name Opts >] {
                        params: self.params.clone(),
                    }
                }
                }
            }
        }
    };
}

/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => (
        $(
            #[doc = $doc]
            $thing
        )*
    );
}

macro_rules! impl_api_ty {
    ($($docs:literal)* $name:ident => $name_field:ident : $name_field_tt:tt) => {
        paste::item! {
            calculated_doc!{
            #[doc = concat!("Interface for accessing and manipulating Docker ", stringify!($name), ".\n", $($docs,)* "\n[Api Reference](https://docs.docker.com/engine/api/", version!() ,"/#tag/", stringify!($name), ")")]
            #[derive(Debug)]
            pub struct [< $name >]<'docker> {
                docker: &'docker crate::Docker,
                $name_field: String,
            }
            }
            impl<'docker> [< $name >]<'docker> {
                // TODO: this is possible on nightly, figure out what to do
                calculated_doc!{
                #[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), " instance.")]
                pub fn new<$name_field_tt>(docker: &'docker crate::Docker, $name_field: $name_field_tt) -> Self
                where
                    $name_field_tt: Into<String>,
                {
                    [< $name >] {
                        docker,
                        $name_field: $name_field.into(),
                    }
                }
                }

                calculated_doc!{
                #[doc = concat!("A getter for ", stringify!($name), " ", stringify!($name_field))]
                pub fn $name_field(&self) -> &str {
                    &self.$name_field
                }
                }


            }


            calculated_doc!{
            #[doc = concat!("Interface for Docker ", stringify!($name), "s.", stringify!($name), ">")]
            #[derive(Debug)]
            pub struct [< $name s >]<'docker> {
                docker: &'docker crate::Docker,
            }
            }

            impl<'docker> [< $name s >]<'docker> {
                calculated_doc!{
                #[doc = concat!("Exports an interface for interacting with Docker ", stringify!($name), "s.")]
                pub fn new(docker: &'docker crate::Docker) -> Self {
                    [< $name s >] { docker }
                }
                }

                calculated_doc!{
                #[doc = concat!("Returns a reference to a set of operations available to a specific ", stringify!($name), ".")]
                pub fn get<$name_field_tt>(&self, $name_field: $name_field_tt) -> [< $name >]<'docker>
                where
                    $name_field_tt: Into<String>,
                {
                    [< $name >]::new(self.docker, $name_field)
                }
                }
            }

        }
    }
}

macro_rules! impl_filter_func {
    ($filter_ty:ident) => {
        pub fn filter<F>(&mut self, filters: F) -> &mut Self
        where
            F: IntoIterator<Item = $filter_ty>,
        {
            let mut param = std::collections::HashMap::new();
            for (key, val) in filters.into_iter().map(|f| f.query_key_val()) {
                param.insert(key, val);
            }
            // structure is a a json encoded object mapping string keys to a list
            // of string values
            self.params
                .insert("filters", serde_json::to_string(&param).unwrap_or_default());
            self
        }
    };
}
