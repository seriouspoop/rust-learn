mod data_type;
mod control_flow;
mod functions;
mod ownership;
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

   // Ownershipb
   ownership::borrowing();
   ownership::moving();
   ownership::references();
}
