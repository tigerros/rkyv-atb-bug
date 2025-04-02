use rkyv::{Archive, Serialize, Deserialize};

// compiles
#[derive(Clone)]
struct WorkingArchivedFoo<T>(T::Archived) where T: Archive<Archived: Clone>;

// uses <T as Archive>::Archived so doesn't compile
#[derive(Clone, Archive, Serialize, Deserialize)]
#[rkyv(derive(Clone))]
struct Foo<T>(T);

fn main() {}