use std::collections::HashMap;

type Index = usize;
type Dist = usize;
type Rank = usize;

// entries (and therefore parents) are 0..max_entry_number. This makes things easy.
// ranks are non-negative integers.
// Translator translates between actual items and their corresponding
// index in the data structure
pub struct UnionFind<T> {
    parents: Vec<Index>,
    ranks: Vec<Rank>,
    translator: T,
}

pub trait Translator {
    type Name;

    fn name_to_index(&self, name: &Self::Name) -> Index;
    fn index_to_name(&self, index: Index) -> &Self::Name;
}

#[derive(Debug, PartialEq, Eq)]
pub struct StringTranslator {
    names: Vec<String>,
    indices: HashMap<String, Index>,
}

impl Translator for StringTranslator {
    type Name = String;

    fn name_to_index(&self, name: &Self::Name) -> Index {
        *self.indices.get(name).unwrap()
    }

    fn index_to_name(&self, index: Index) -> &Self::Name {
        &self.names[index]
    }
}

impl<T> UnionFind<T>
where
    T: Translator,
{
    pub fn find(&mut self, vertex: &<T as Translator>::Name) -> &T {
        let mut current_index = self.translator.name_to_index(vertex);
        let mut parent_index = self.parents[current_index];
        let mut todo = vec![];
        let mut at_root = current_index == parent_index;

        while !at_root {
            todo.push(current_index);
            current_index = parent_index;
            parent_index = self.parents[current_index];
            at_root = parent_index == current_index;
        }

        let root = parent_index;

        todo!()
    }
}
