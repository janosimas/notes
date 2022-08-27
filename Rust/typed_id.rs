use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

/// Strong typed generic id
///
/// # Example
///
/// ```rust
/// struct User;
/// type UserId = TypedId<u32, User>;
/// ```
pub struct TypedId<U, T>(pub U, PhantomData<T>);
pub type TypedUuid<T> = TypedId<uuid::Uuid, T>;

impl<U, T> TypedId<U, T> {
    pub fn new(id: U) -> Self {
        Self(id, PhantomData)
    }
}

impl<U: Default, T> Default for TypedId<U, T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<U: Debug, T> Debug for TypedId<U, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypeId")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<U: Clone, T> Clone for TypedId<U, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<U: Copy, T> Copy for TypedId<U, T> {}

impl<U: Hash, T> Hash for TypedId<U, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<U: PartialEq, T> PartialEq for TypedId<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<U: Eq, T> Eq for TypedId<U, T> {}

impl<U, T> Deref for TypedId<U, T> {
    type Target = U;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U, T> From<U> for TypedId<U, T> {
    fn from(other: U) -> TypedId<U, T> {
        TypedId(other, PhantomData)
    }
}

impl<U: PartialOrd, T> PartialOrd for TypedId<U, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<U: Ord, T> Ord for TypedId<U, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
