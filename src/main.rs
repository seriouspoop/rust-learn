mod data_type;
mod control_flow;
mod functions;
mod ownership;
mod structs;

fn main() {
   // Data types
   data_type::primitives();
   data_type::strings();
   data_type::arrays();
   data_type::tuples();
   data_type::vectors();

   // Control Flow
   control_flow::conditionals();
   control_flow::loops();

   // Functions
   functions::example_function();

   // Ownership
   ownership::borrowing();
   ownership::moving();
   ownership::references();
   ownership::slice_referencing();

   // Structs
   structs::sample_struct();
   structs::update_syntax();
   structs::tuple_struct();
   structs::unit_structs();
   structs::methods();
}
