#[macro_use]
extern crate druid_xml_macro;

pub fn main() {
    struct MyStruct;
    druid_xml!(
        "
        <style>
        flex { background-color:black; }
        <style>

        ",
        "flex" => |w| {
            
        },
        "button" => |b| {

        });
}