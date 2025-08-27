// #[macro_export]
// macro_rules! plugin_group {
//     {
//         $(#[$group_meta:meta])*
//         $vis:vis struct $group:ident {
//             $(
//                 $(#[cfg(feature = $plugin_feature:literal)])?
//                 $(#[custom($plugin_meta:meta)])*
//                 $($plugin_path:ident::)* : $plugin_name:ident
//             ),*
//             $(
//                 $(,)?$(
//                     #[plugin_group]
//                     $(#[cfg(feature = $plugin_group_feature:literal)])?
//                     $(#[custom($plugin_group_meta:meta)])*
//                     $($plugin_group_path:ident::)* : $plugin_group_name:ident
//                 ),+
//             )?
//             $(
//                 $(,)?$(
//                     #[doc(hidden)]
//                     $(#[cfg(feature = $hidden_plugin_feature:literal)])?
//                     $(#[custom($hidden_plugin_meta:meta)])*
//                     $($hidden_plugin_path:ident::)* : $hidden_plugin_name:ident
//                 ),+
//             )?

//             $(,)?
//         }
//         $($(#[doc = $post_doc:literal])+)?
//     } => {
//         $(#[$group_meta])*
//         ///
//         $(#[doc = concat!(
//             " - [`", stringify!($plugin_name), "`](" $(, stringify!($plugin_path), "::")*, stringify!($plugin_name), ")"
//             $(, " - with feature `", $plugin_feature, "`")?
//         )])*
//        $($(#[doc = concat!(
//             " - [`", stringify!($plugin_group_name), "`](" $(, stringify!($plugin_group_path), "::")*, stringify!($plugin_group_name), ")"
//             $(, " - with feature `", $plugin_group_feature, "`")?
//         )]),+)?
//         $(
//             ///
//             $(#[doc = $post_doc])+
//         )?
//         $vis struct $group;

//         impl $crate::PluginGroup for $group {
//             fn build(self) -> $crate::PluginGroupBuilder {
//                 let mut group = $crate::PluginGroupBuilder::start::<Self>();

//                 $(
//                     $(#[cfg(feature = $plugin_feature)])?
//                     $(#[$plugin_meta])*
//                     {
//                         const _: () = {
//                             const fn check_default<T: Default>() {}
//                             check_default::<$($plugin_path::)*$plugin_name>();
//                         };

//                         group = group.add(<$($plugin_path::)*$plugin_name>::default());
//                     }
//                 )*
//                 $($(
//                     $(#[cfg(feature = $plugin_group_feature)])?
//                     $(#[$plugin_group_meta])*
//                     {
//                         const _: () = {
//                             const fn check_default<T: Default>() {}
//                             check_default::<$($plugin_group_path::)*$plugin_group_name>();
//                         };

//                         group = group.add_group(<$($plugin_group_path::)*$plugin_group_name>::default());
//                     }
//                 )+)?
//                 $($(
//                     $(#[cfg(feature = $hidden_plugin_feature)])?
//                     $(#[$hidden_plugin_meta])*
//                     {
//                         const _: () = {
//                             const fn check_default<T: Default>() {}
//                             check_default::<$($hidden_plugin_path::)*$hidden_plugin_name>();
//                         };

//                         group = group.add(<$($hidden_plugin_path::)*$hidden_plugin_name>::default());
//                     }
//                 )+)?

//                 group
//             }
//         }
//     };
// }