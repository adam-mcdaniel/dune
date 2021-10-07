use common_macros::b_tree_map;
use dune::Expression;

pub fn get() -> Expression {
    let os = os_info::get();
    let os_type = os.os_type();

    (b_tree_map! {
        String::from("name") => Expression::from(crate::get_os_name(&os_type)),
        String::from("family") => crate::get_os_family(&os_type).into(),
        String::from("version") => os.version().to_string().into(),
    })
    .into()
}
