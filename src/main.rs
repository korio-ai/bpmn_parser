mod bpmn;

use std::fs::File;
use std::io::Read;
use bpmn::Definitions;

fn main() {
    // Read BPMN XML from file
    let mut file = File::open("bpmn.xml").expect("Unable to open BPMN XML file");
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data).expect("Unable to read BPMN XML file");

    // Parse the XML data
    let definitions: Definitions = quick_xml::de::from_reader(xml_data.as_bytes()).expect("Failed to parse BPMN XML");

    // Print the parsed definitions
    println!("{:#?}", definitions);
}
