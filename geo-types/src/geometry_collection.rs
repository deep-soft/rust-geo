use crate::{CoordNum, Geometry};

#[cfg(any(feature = "approx", test))]
use approx::{AbsDiffEq, RelativeEq};
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

/// A collection of [`Geometry`](enum.Geometry.html) types.
///
/// It can be created from a `Vec` of Geometries, or from an Iterator which yields Geometries.
///
/// Looping over this object yields its component **Geometry
/// enum members** (_not_ the underlying geometry
/// primitives), and it supports iteration and indexing as
/// well as the various
/// [`MapCoords`](algorithm/map_coords/index.html)
/// functions, which _are_ directly applied to the
/// underlying geometry primitives.
///
/// # Examples
/// ## Looping
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe = Geometry::Point(p);
/// let gc = GeometryCollection::new_from(vec![pe]);
/// for geom in gc {
///     println!("{:?}", Point::try_from(geom).unwrap().x());
/// }
/// ```
/// ## Implements `iter()`
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe = Geometry::Point(p);
/// let gc = GeometryCollection::new_from(vec![pe]);
/// gc.iter().for_each(|geom| println!("{:?}", geom));
/// ```
///
/// ## Mutable Iteration
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe = Geometry::Point(p);
/// let mut gc = GeometryCollection::new_from(vec![pe]);
/// gc.iter_mut().for_each(|geom| {
///    if let Geometry::Point(p) = geom {
///        p.set_x(0.2);
///    }
/// });
/// let updated = gc[0].clone();
/// assert_eq!(Point::try_from(updated).unwrap().x(), 0.2);
/// ```
///
/// ## Indexing
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe = Geometry::Point(p);
/// let gc = GeometryCollection::new_from(vec![pe]);
/// println!("{:?}", gc[0]);
/// ```
///
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryCollection<T: CoordNum>(
    #[deprecated(
        since = "0.7.5",
        note = "Direct field access is deprecated - use `geometry_collection.geometries()`, `geometry_collection.geometries_mut()`, or `geometry_collection[idx]` for field access and `GeometryCollection::from(geometry_vec)` for construction"
    )]
    pub Vec<Geometry<T>>,
);

// Implementing Default by hand because T does not have Default restriction
// todo: consider adding Default as a CoordNum requirement
impl<T: CoordNum> Default for GeometryCollection<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// Create a `GeometryCollection` from a `Vec` of geometries.
/// # Examples
/// ```
/// use geo_types::{point, line_string, Geometry, GeometryCollection};
///
/// let point = Geometry::from(point!(x: 1.0, y: 2.0));
/// let line_string = Geometry::from(line_string![(x: 0.0, y: 0.0), (x: 2.0, y: 0.0), (x: 2.0, y: 2.0)]);
/// let collection = GeometryCollection::from(vec![point, line_string]);
/// ```
impl<T: CoordNum> From<Vec<Geometry<T>>> for GeometryCollection<T> {
    /// Create a `GeometryCollection` with geometries as its members.
    fn from(geometries: Vec<Geometry<T>>) -> Self {
        GeometryCollection(geometries)
    }
}

impl<T: CoordNum> GeometryCollection<T> {
    /// Return an empty GeometryCollection
    #[deprecated(
        note = "Will be replaced with a parametrized version in upcoming version. Use GeometryCollection::default() instead"
    )]
    pub fn new() -> Self {
        GeometryCollection::default()
    }

    /// DO NOT USE!
    /// This fn will be renamed to `new` in the upcoming version.
    /// This fn is not marked as deprecated because it would require extensive refactoring of the geo code.
    pub fn new_from(value: Vec<Geometry<T>>) -> Self {
        Self(value)
    }

    /// Get the constituent geometries of this collection.
    pub fn geometries(&self) -> &[Geometry<T>] {
        #[allow(deprecated)]
        &self.0
    }

    /// Mutable borrow the constituent geometries of this collection.
    pub fn geometries_mut(&mut self) -> &mut [Geometry<T>] {
        #[allow(deprecated)]
        &mut self.0
    }

    /// Consume this collection to get ownership of the constituent geometries.
    pub fn into_inner(self) -> Vec<Geometry<T>> {
        #[allow(deprecated)]
        self.0
    }

    /// Number of geometries in this GeometryCollection
    pub fn len(&self) -> usize {
        self.geometries().len()
    }

    /// Is this GeometryCollection empty
    pub fn is_empty(&self) -> bool {
        self.geometries().is_empty()
    }

    /// Push `geometry` onto the end of the collection.
    pub fn push(&mut self, geometry: Geometry<T>) {
        #[allow(deprecated)]
        self.0.push(geometry)
    }
}

/// Convert any Geometry (or anything that can be converted to a Geometry) into a
/// GeometryCollection
impl<T: CoordNum, IG: Into<Geometry<T>>> From<IG> for GeometryCollection<T> {
    fn from(x: IG) -> Self {
        Self(vec![x.into()])
    }
}

/// Collect Geometries (or what can be converted to a Geometry) into a GeometryCollection
impl<T: CoordNum, IG: Into<Geometry<T>>> FromIterator<IG> for GeometryCollection<T> {
    fn from_iter<I: IntoIterator<Item = IG>>(iter: I) -> Self {
        Self(iter.into_iter().map(|g| g.into()).collect())
    }
}

impl<T: CoordNum> Index<usize> for GeometryCollection<T> {
    type Output = Geometry<T>;

    fn index(&self, index: usize) -> &Geometry<T> {
        self.geometries().index(index)
    }
}

impl<T: CoordNum> IndexMut<usize> for GeometryCollection<T> {
    fn index_mut(&mut self, index: usize) -> &mut Geometry<T> {
        self.geometries_mut().index_mut(index)
    }
}

// structure helper for consuming iterator
#[derive(Debug)]
pub struct IntoIteratorHelper<T: CoordNum> {
    iter: ::std::vec::IntoIter<Geometry<T>>,
}

// implement the IntoIterator trait for a consuming iterator. Iteration will
// consume the GeometryCollection
impl<T: CoordNum> IntoIterator for GeometryCollection<T> {
    type Item = Geometry<T>;
    type IntoIter = IntoIteratorHelper<T>;

    // note that into_iter() is consuming self
    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorHelper {
            iter: self.into_inner().into_iter(),
        }
    }
}

// implement Iterator trait for the helper struct, to be used by adapters
impl<T: CoordNum> Iterator for IntoIteratorHelper<T> {
    type Item = Geometry<T>;

    // just return the reference
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// structure helper for non-consuming iterator
#[derive(Debug)]
pub struct IterHelper<'a, T: CoordNum> {
    iter: ::std::slice::Iter<'a, Geometry<T>>,
}

// implement the IntoIterator trait for a non-consuming iterator. Iteration will
// borrow the GeometryCollection
impl<'a, T: CoordNum> IntoIterator for &'a GeometryCollection<T> {
    type Item = &'a Geometry<T>;
    type IntoIter = IterHelper<'a, T>;

    // note that into_iter() is consuming self
    fn into_iter(self) -> Self::IntoIter {
        IterHelper {
            iter: self.geometries().iter(),
        }
    }
}

// implement the Iterator trait for the helper struct, to be used by adapters
impl<'a, T: CoordNum> Iterator for IterHelper<'a, T> {
    type Item = &'a Geometry<T>;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// structure helper for mutable non-consuming iterator
#[derive(Debug)]
pub struct IterMutHelper<'a, T: CoordNum> {
    iter: ::std::slice::IterMut<'a, Geometry<T>>,
}

// implement the IntoIterator trait for a mutable non-consuming iterator. Iteration will
// mutably borrow the GeometryCollection
impl<'a, T: CoordNum> IntoIterator for &'a mut GeometryCollection<T> {
    type Item = &'a mut Geometry<T>;
    type IntoIter = IterMutHelper<'a, T>;

    // note that into_iter() is consuming self
    fn into_iter(self) -> Self::IntoIter {
        IterMutHelper {
            iter: self.geometries_mut().iter_mut(),
        }
    }
}

// implement the Iterator trait for the helper struct, to be used by adapters
impl<'a, T: CoordNum> Iterator for IterMutHelper<'a, T> {
    type Item = &'a mut Geometry<T>;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T: CoordNum> GeometryCollection<T> {
    pub fn iter(&'a self) -> IterHelper<'a, T> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> IterMutHelper<'a, T> {
        self.into_iter()
    }
}

#[cfg(any(feature = "approx", test))]
impl<T> RelativeEq for GeometryCollection<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordNum + RelativeEq,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    /// Equality assertion within a relative limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{GeometryCollection, point};
    ///
    /// let a = GeometryCollection::new_from(vec![point![x: 1.0, y: 2.0].into()]);
    /// let b = GeometryCollection::new_from(vec![point![x: 1.0, y: 2.01].into()]);
    ///
    /// approx::assert_relative_eq!(a, b, max_relative=0.1);
    /// approx::assert_relative_ne!(a, b, max_relative=0.0001);
    /// ```
    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut mp_zipper = self.iter().zip(other.iter());
        mp_zipper.all(|(lhs, rhs)| lhs.relative_eq(rhs, epsilon, max_relative))
    }
}

#[cfg(any(feature = "approx", test))]
impl<T> AbsDiffEq for GeometryCollection<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordNum,
    T::Epsilon: Copy,
{
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    /// Equality assertion with an absolute limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{GeometryCollection, point};
    ///
    /// let a = GeometryCollection::new_from(vec![point![x: 0.0, y: 0.0].into()]);
    /// let b = GeometryCollection::new_from(vec![point![x: 0.0, y: 0.1].into()]);
    ///
    /// approx::abs_diff_eq!(a, b, epsilon=0.1);
    /// approx::abs_diff_ne!(a, b, epsilon=0.001);
    /// ```
    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut mp_zipper = self.into_iter().zip(other.into_iter());
        mp_zipper.all(|(lhs, rhs)| lhs.abs_diff_eq(rhs, epsilon))
    }
}
