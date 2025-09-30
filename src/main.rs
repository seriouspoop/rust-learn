mod chapter1;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;

fn main() {
    // Data types
    chapter1::data_types::primitives();
    chapter1::data_types::strings();
    chapter1::data_types::arrays();
    chapter1::data_types::tuples();
    chapter1::data_types::vectors();

    // Control Flow
    chapter2::control_flow::conditionals();
    chapter2::control_flow::loops();

    // Functions
    chapter3::functions::example_function();

    // Ownership
    chapter4::ownership::borrowing();
    chapter4::ownership::moving();
    chapter4::ownership::references();
    chapter4::ownership::slice_referencing();

    // Structs
    chapter5::structs::sample_struct();
    chapter5::structs::update_syntax();
    chapter5::structs::tuple_struct();
    chapter5::structs::unit_structs();
    chapter5::structs::methods();

    chapter6::enums::ip_addr_example();
    chapter6::enums::option_enum();
}
