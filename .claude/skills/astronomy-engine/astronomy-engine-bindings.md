# Crate Documentation

**Version:** 2.1.19

**Format Version:** 57

# Module `astronomy_engine_bindings`

## Types

### Type Alias `wchar_t`

```rust
pub type wchar_t = ::std::os::raw::c_int;
```

### Struct `max_align_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: Some(16), packed: None, int: None })`

```rust
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `__clang_max_align_nonce1` | `::std::os::raw::c_longlong` |  |
| `__bindgen_padding_0` | `u64` |  |
| `__clang_max_align_nonce2` | `u128` |  |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> max_align_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_status_t`

@brief Indicates success/failure of an Astronomy Engine function call.

```rust
pub type astro_status_t = ::std::os::raw::c_uint;
```

### Struct `astro_time_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A date and time used for astronomical calculations.

This type is of fundamental importance to Astronomy Engine.
It is used to represent dates and times for all astronomical calculations.
It is also included in the values returned by many Astronomy Engine functions.

To create a valid astro_time_t value from scratch, call #Astronomy_MakeTime
(for a given calendar date and time) or #Astronomy_CurrentTime (for the system's
current date and time).

To adjust an existing astro_time_t by a certain real number of days,
call #Astronomy_AddDays.

The astro_time_t type contains `ut` to represent Universal Time (UT1/UTC) and
`tt` to represent Terrestrial Time (TT, also known as *ephemeris time*).
The difference `tt-ut` is known as *&Delta;T*, using a best-fit piecewise model devised by
[Espenak and Meeus](https://eclipse.gsfc.nasa.gov/SEhelp/deltatpoly2004.html).

Both `tt` and `ut` are necessary for performing different astronomical calculations.
Indeed, certain calculations (such as rise/set times) require both time scales.
See the documentation for the `ut` and `tt` fields for more detailed information.

In cases where `astro_time_t` is included in a structure returned by
a function that can fail, the `astro_status_t` field `status` will contain a value
other than `ASTRO_SUCCESS`; in that case the `ut` and `tt` will hold `NAN` (not a number).
In general, when there is an error code stored in a struct field `status`, the
caller should ignore all other values in that structure, including the `ut` and `tt`
inside `astro_time_t`.

```rust
pub struct astro_time_t {
    pub ut: f64,
    pub tt: f64,
    pub psi: f64,
    pub eps: f64,
    pub st: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ut` | `f64` | @brief   UT1/UTC number of days since noon on January 1, 2000.<br><br>The floating point number of days of Universal Time since noon UTC January 1, 2000.<br>Astronomy Engine approximates UTC and UT1 as being the same thing, although they are<br>not exactly equivalent; UTC and UT1 can disagree by up to &plusmn;0.9 seconds.<br>This approximation is sufficient for the accuracy requirements of Astronomy Engine.<br><br>Universal Time Coordinate (UTC) is the international standard for legal and civil<br>timekeeping and replaces the older Greenwich Mean Time (GMT) standard.<br>UTC is kept in sync with unpredictable observed changes in the Earth's rotation<br>by occasionally adding leap seconds as needed.<br><br>UT1 is an idealized time scale based on observed rotation of the Earth, which<br>gradually slows down in an unpredictable way over time, due to tidal drag by the Moon and Sun,<br>large scale weather events like hurricanes, and internal seismic and convection effects.<br>Conceptually, UT1 drifts from atomic time continuously and erratically, whereas UTC<br>is adjusted by a scheduled whole number of leap seconds as needed.<br><br>The value in `ut` is appropriate for any calculation involving the Earth's rotation,<br>such as calculating rise/set times, culumination, and anything involving apparent<br>sidereal time.<br><br>Before the era of atomic timekeeping, days based on the Earth's rotation<br>were often known as *mean solar days*. |
| `tt` | `f64` | @brief   Terrestrial Time days since noon on January 1, 2000.<br><br>Terrestrial Time is an atomic time scale defined as a number of days since noon on January 1, 2000.<br>In this system, days are not based on Earth rotations, but instead by<br>the number of elapsed [SI seconds](https://physics.nist.gov/cuu/Units/second.html)<br>divided by 86400. Unlike `ut`, `tt` increases uniformly without adjustments<br>for changes in the Earth's rotation.<br><br>The value in `tt` is used for calculations of movements not involving the Earth's rotation,<br>such as the orbits of planets around the Sun, or the Moon around the Earth.<br><br>Historically, Terrestrial Time has also been known by the term *Ephemeris Time* (ET). |
| `psi` | `f64` | @brief   For internal use only. Used to optimize Earth tilt calculations. |
| `eps` | `f64` | @brief   For internal use only.  Used to optimize Earth tilt calculations. |
| `st` | `f64` | @brief   For internal use only.  Lazy-caches sidereal time (Earth rotation). |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_time_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_utc_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A calendar date and time expressed in UTC.

```rust
pub struct astro_utc_t {
    pub year: ::std::os::raw::c_int,
    pub month: ::std::os::raw::c_int,
    pub day: ::std::os::raw::c_int,
    pub hour: ::std::os::raw::c_int,
    pub minute: ::std::os::raw::c_int,
    pub second: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `year` | `::std::os::raw::c_int` | < The year value, e.g. 2019. |
| `month` | `::std::os::raw::c_int` | < The month value: 1=January, 2=February, ..., 12=December. |
| `day` | `::std::os::raw::c_int` | < The day of the month in the range 1..31. |
| `hour` | `::std::os::raw::c_int` | < The hour of the day in the range 0..23. |
| `minute` | `::std::os::raw::c_int` | < The minute of the hour in the range 0..59. |
| `second` | `f64` | < The floating point number of seconds in the range [0,60). |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_utc_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_vector_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A 3D Cartesian vector whose components are expressed in Astronomical Units (AU).

```rust
pub struct astro_vector_t {
    pub status: astro_status_t,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: astro_time_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `x` | `f64` | < The Cartesian x-coordinate of the vector in AU. |
| `y` | `f64` | < The Cartesian y-coordinate of the vector in AU. |
| `z` | `f64` | < The Cartesian z-coordinate of the vector in AU. |
| `t` | `astro_time_t` | < The date and time at which this vector is valid. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_vector_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_state_vector_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A state vector that contains a position (AU) and velocity (AU/day).

```rust
pub struct astro_state_vector_t {
    pub status: astro_status_t,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub t: astro_time_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `x` | `f64` | < The Cartesian position x-coordinate of the vector in AU. |
| `y` | `f64` | < The Cartesian position y-coordinate of the vector in AU. |
| `z` | `f64` | < The Cartesian position z-coordinate of the vector in AU. |
| `vx` | `f64` | < The Cartesian velocity x-coordinate of the vector in AU/day. |
| `vy` | `f64` | < The Cartesian velocity y-coordinate of the vector in AU/day. |
| `vz` | `f64` | < The Cartesian velocity z-coordinate of the vector in AU/day. |
| `t` | `astro_time_t` | < The date and time at which this state vector is valid. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_state_vector_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_spherical_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Spherical coordinates: latitude, longitude, distance.

```rust
pub struct astro_spherical_t {
    pub status: astro_status_t,
    pub lat: f64,
    pub lon: f64,
    pub dist: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `lat` | `f64` | < The latitude angle: -90..+90 degrees. |
| `lon` | `f64` | < The longitude angle: 0..360 degrees. |
| `dist` | `f64` | < Distance in AU. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_spherical_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_angle_result_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief An angular value expressed in degrees.

```rust
pub struct astro_angle_result_t {
    pub status: astro_status_t,
    pub angle: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `angle` | `f64` | < An angle expressed in degrees. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_angle_result_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_body_t`

@brief A celestial body.

```rust
pub type astro_body_t = ::std::os::raw::c_int;
```

### Struct `astro_observer_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief The location of an observer on (or near) the surface of the Earth.

This structure is passed to functions that calculate phenomena as observed
from a particular place on the Earth.

You can create this structure directly, or you can call the convenience function
#Astronomy_MakeObserver to create one for you.

```rust
pub struct astro_observer_t {
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `latitude` | `f64` | < Geographic latitude in degrees north (positive) or south (negative) of the equator. |
| `longitude` | `f64` | < Geographic longitude in degrees east (positive) or west (negative) of the prime meridian at Greenwich, England. |
| `height` | `f64` | < The height above (positive) or below (negative) sea level, expressed in meters. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_observer_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_equatorial_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Equatorial angular and cartesian coordinates.

Coordinates of a celestial body as seen from the Earth (geocentric or topocentric, depending on context),
oriented with respect to the projection of the Earth's equator onto the sky.

```rust
pub struct astro_equatorial_t {
    pub status: astro_status_t,
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
    pub vec: astro_vector_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `ra` | `f64` | < right ascension in sidereal hours. |
| `dec` | `f64` | < declination in degrees |
| `dist` | `f64` | < distance to the celestial body in AU. |
| `vec` | `astro_vector_t` | < equatorial coordinates in cartesian vector form: x = March equinox, y = June solstice, z = north. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_equatorial_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_ecliptic_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Ecliptic angular and Cartesian coordinates.

Coordinates of a celestial body as seen from the center of the Sun (heliocentric),
oriented with respect to the plane of the Earth's orbit around the Sun (the ecliptic).

```rust
pub struct astro_ecliptic_t {
    pub status: astro_status_t,
    pub vec: astro_vector_t,
    pub elat: f64,
    pub elon: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `vec` | `astro_vector_t` | < Cartesian ecliptic vector: x=equinox, y=90 degrees prograde in ecliptic plane, z=northward perpendicular to ecliptic. |
| `elat` | `f64` | < Latitude in degrees north (positive) or south (negative) of the ecliptic plane. |
| `elon` | `f64` | < Longitude in degrees around the ecliptic plane prograde from the equinox. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_ecliptic_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_horizon_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Coordinates of a celestial body as seen by a topocentric observer.

Contains horizontal and equatorial coordinates seen by an observer on or near
the surface of the Earth (a topocentric observer).
Optionally corrected for atmospheric refraction.

```rust
pub struct astro_horizon_t {
    pub azimuth: f64,
    pub altitude: f64,
    pub ra: f64,
    pub dec: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `azimuth` | `f64` | < Compass direction around the horizon in degrees. 0=North, 90=East, 180=South, 270=West. |
| `altitude` | `f64` | < Angle in degrees above (positive) or below (negative) the observer's horizon. |
| `ra` | `f64` | < Right ascension in sidereal hours. |
| `dec` | `f64` | < Declination in degrees. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_horizon_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_rotation_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Contains a rotation matrix that can be used to transform one coordinate system to another.

```rust
pub struct astro_rotation_t {
    pub status: astro_status_t,
    pub rot: [[f64; 3]; 3],
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `rot` | `[[f64; 3]; 3]` | < A normalized 3x3 rotation matrix. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_rotation_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_refraction_t`

@brief Selects whether to correct for atmospheric refraction, and if so, how.

```rust
pub type astro_refraction_t = ::std::os::raw::c_uint;
```

### Struct `astro_atmosphere_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about idealized atmospheric variables at a given elevation.

```rust
pub struct astro_atmosphere_t {
    pub status: astro_status_t,
    pub pressure: f64,
    pub temperature: f64,
    pub density: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `pressure` | `f64` | < Atmospheric pressure in pascals |
| `temperature` | `f64` | < Atmospheric temperature in kelvins |
| `density` | `f64` | < Atmospheric density relative to sea level |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_atmosphere_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_search_result_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief The result of a search for an astronomical event.

```rust
pub struct astro_search_result_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The time at which a searched-for event occurs. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_search_result_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_seasons_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief
     The dates and times of changes of season for a given calendar year.
     Call #Astronomy_Seasons to calculate this data structure for a given year.

```rust
pub struct astro_seasons_t {
    pub status: astro_status_t,
    pub mar_equinox: astro_time_t,
    pub jun_solstice: astro_time_t,
    pub sep_equinox: astro_time_t,
    pub dec_solstice: astro_time_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `mar_equinox` | `astro_time_t` | < The date and time of the March equinox for the specified year. |
| `jun_solstice` | `astro_time_t` | < The date and time of the June soltice for the specified year. |
| `sep_equinox` | `astro_time_t` | < The date and time of the September equinox for the specified year. |
| `dec_solstice` | `astro_time_t` | < The date and time of the December solstice for the specified year. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_seasons_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_moon_quarter_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A lunar quarter event (new moon, first quarter, full moon, or third quarter) along with its date and time.

```rust
pub struct astro_moon_quarter_t {
    pub status: astro_status_t,
    pub quarter: ::std::os::raw::c_int,
    pub time: astro_time_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `quarter` | `::std::os::raw::c_int` | < 0=new moon, 1=first quarter, 2=full moon, 3=third quarter. |
| `time` | `astro_time_t` | < The date and time of the lunar quarter. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_moon_quarter_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_func_result_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief A real value returned by a function whose ascending root is to be found.

When calling #Astronomy_Search, the caller must pass in a callback function
compatible with the function-pointer type #astro_search_func_t
whose ascending root is to be found. That callback function must return astro_func_result_t.
If the function call is successful, it will set `status` to `ASTRO_SUCCESS` and `value`
to the numeric value appropriate for the given date and time.
If the call fails for some reason, it should set `status` to an appropriate error value
other than `ASTRO_SUCCESS`; in the error case, to guard against any possible misuse of `value`,
it is recommended to set `value` to `NAN`, though this is not strictly necessary.

```rust
pub struct astro_func_result_t {
    pub status: astro_status_t,
    pub value: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `value` | `f64` | < The value returned by a function whose ascending root is to be found. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_func_result_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_search_func_t`

@brief A pointer to a function that is to be passed as a callback to #Astronomy_Search.

The function #Astronomy_Search numerically solves for the time that a given event occurs.
An event is defined as the time when an arbitrary function transitions between having
a negative value and a non-negative value. This transition is called an *ascending root*.

The type astro_search_func_t represents such a callback function that accepts a
custom `context` pointer and an astro_time_t representing the time to probe.
The function returns an astro_func_result_t that contains either a real
number in `value` or an error code in `status` that aborts the search.

The `context` points to some data whose type varies depending on the callback function.
It can contain any auxiliary parameters (other than time) needed to evaluate the function.
For example, a function may pertain to a specific celestial body, in which case `context`
may point to a value of type astro_body_t. The `context` parameter is supplied by
the caller of #Astronomy_Search, which passes it along to every call to the callback function.
If the caller of `Astronomy_Search` knows that the callback function does not need a context,
it is safe to pass `NULL` as the context pointer.

```rust
pub type astro_search_func_t = ::std::option::Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void, astro_time_t) -> astro_func_result_t>;
```

### Type Alias `astro_deltat_func`

@brief A pointer to a function that calculates Delta T.

Delta T is the discrepancy between times measured using an atomic clock
and times based on observations of the Earth's rotation, which is gradually
slowing down over time. Delta T = TT - UT, where
TT = Terrestrial Time, based on atomic time, and
UT = Universal Time, civil time based on the Earth's rotation.
Astronomy Engine defaults to using a Delta T function defined by
Espenak and Meeus in their "Five Millennium Canon of Solar Eclipses".
See: https://eclipse.gsfc.nasa.gov/SEhelp/deltatpoly2004.html

```rust
pub type astro_deltat_func = ::std::option::Option<unsafe extern "C" fn(f64) -> f64>;
```

### Type Alias `astro_visibility_t`

@brief Indicates whether a body (especially Mercury or Venus) is best seen in the morning or evening.

```rust
pub type astro_visibility_t = ::std::os::raw::c_uint;
```

### Struct `astro_elongation_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief
     Contains information about the visibility of a celestial body at a given date and time.
     See #Astronomy_Elongation for more detailed information about the members of this structure.
     See also #Astronomy_SearchMaxElongation for how to search for maximum elongation events.

```rust
pub struct astro_elongation_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub visibility: astro_visibility_t,
    pub elongation: f64,
    pub ecliptic_separation: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The date and time of the observation. |
| `visibility` | `astro_visibility_t` | < Whether the body is best seen in the morning or the evening. |
| `elongation` | `f64` | < The angle in degrees between the body and the Sun, as seen from the Earth. |
| `ecliptic_separation` | `f64` | < The difference between the ecliptic longitudes of the body and the Sun, as seen from the Earth. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_elongation_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_hour_angle_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about a celestial body crossing a specific hour angle.

Returned by the function #Astronomy_SearchHourAngleEx to report information about
a celestial body crossing a certain hour angle as seen by a specified topocentric observer.

```rust
pub struct astro_hour_angle_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub hor: astro_horizon_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The date and time when the body crosses the specified hour angle. |
| `hor` | `astro_horizon_t` | < Apparent coordinates of the body at the time it crosses the specified hour angle. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_hour_angle_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_illum_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about the brightness and illuminated shape of a celestial body.

Returned by the functions #Astronomy_Illumination and #Astronomy_SearchPeakMagnitude
to report the visual magnitude and illuminated fraction of a celestial body at a given date and time.

```rust
pub struct astro_illum_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub mag: f64,
    pub phase_angle: f64,
    pub phase_fraction: f64,
    pub helio_dist: f64,
    pub ring_tilt: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The date and time of the observation. |
| `mag` | `f64` | < The visual magnitude of the body. Smaller values are brighter. |
| `phase_angle` | `f64` | < The angle in degrees between the Sun and the Earth, as seen from the body. Indicates the body's phase as seen from the Earth. |
| `phase_fraction` | `f64` | < A value in the range [0.0, 1.0] indicating what fraction of the body's apparent disc is illuminated, as seen from the Earth. |
| `helio_dist` | `f64` | < The distance between the Sun and the body at the observation time. |
| `ring_tilt` | `f64` | < For Saturn, the tilt angle in degrees of its rings as seen from Earth. For all other bodies, 0. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_illum_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_apsis_kind_t`

@brief The type of apsis: pericenter (closest approach) or apocenter (farthest distance).

```rust
pub type astro_apsis_kind_t = ::std::os::raw::c_uint;
```

### Struct `astro_apsis_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief An apsis event: pericenter (closest approach) or apocenter (farthest distance).

For the Moon orbiting the Earth, or a planet orbiting the Sun, an *apsis* is an
event where the orbiting body reaches its closest or farthest point from the primary body.
The closest approach is called *pericenter* and the farthest point is *apocenter*.

More specific terminology is common for particular orbiting bodies.
The Moon's closest approach to the Earth is called *perigee* and its farthest
point is called *apogee*. The closest approach of a planet to the Sun is called
*perihelion* and the furthest point is called *aphelion*.

This data structure is returned by #Astronomy_SearchLunarApsis and #Astronomy_NextLunarApsis
to iterate through consecutive alternating perigees and apogees.

```rust
pub struct astro_apsis_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub kind: astro_apsis_kind_t,
    pub dist_au: f64,
    pub dist_km: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The date and time of the apsis. |
| `kind` | `astro_apsis_kind_t` | < Whether this is a pericenter or apocenter event. |
| `dist_au` | `f64` | < The distance between the centers of the bodies in astronomical units. |
| `dist_km` | `f64` | < The distance between the centers of the bodies in kilometers. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_apsis_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_eclipse_kind_t`

@brief The different kinds of lunar/solar eclipses.

```rust
pub type astro_eclipse_kind_t = ::std::os::raw::c_uint;
```

### Struct `astro_lunar_eclipse_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about a lunar eclipse.

Returned by #Astronomy_SearchLunarEclipse or #Astronomy_NextLunarEclipse
to report information about a lunar eclipse event.
If a lunar eclipse is found, `status` holds `ASTRO_SUCCESS` and the other fields are set.
If `status` holds any other value, it is an error code and the other fields are undefined.

When a lunar eclipse is found, it is classified as penumbral, partial, or total.
Penumbral eclipses are difficult to observe, because the Moon is only slightly dimmed
by the Earth's penumbra; no part of the Moon touches the Earth's umbra.
Partial eclipses occur when part, but not all, of the Moon touches the Earth's umbra.
Total eclipses occur when the entire Moon passes into the Earth's umbra.

The `kind` field thus holds `ECLIPSE_PENUMBRAL`, `ECLIPSE_PARTIAL`, or `ECLIPSE_TOTAL`,
depending on the kind of lunar eclipse found.

The `obscuration` field holds a value in the range [0, 1] that indicates what fraction
of the Moon's apparent disc area is covered by the Earth's umbra at the eclipse's peak.
This indicates how dark the peak eclipse appears. For penumbral eclipses, the obscuration
is 0, because the Moon does not pass through the Earth's umbra. For partial eclipses,
the obscuration is somewhere between 0 and 1. For total lunar eclipses, the obscuration is 1.

Field `peak` holds the date and time of the center of the eclipse, when it is at its peak.

Fields `sd_penum`, `sd_partial`, and `sd_total` hold the semi-duration of each phase
of the eclipse, which is half of the amount of time the eclipse spends in each
phase (expressed in minutes), or 0 if the eclipse never reaches that phase.
By converting from minutes to days, and subtracting/adding with `center`, the caller
may determine the date and time of the beginning/end of each eclipse phase.

```rust
pub struct astro_lunar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub obscuration: f64,
    pub peak: astro_time_t,
    pub sd_penum: f64,
    pub sd_partial: f64,
    pub sd_total: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `kind` | `astro_eclipse_kind_t` | < The type of lunar eclipse found. |
| `obscuration` | `f64` | < The peak fraction of the Moon's apparent disc that is covered by the Earth's umbra. |
| `peak` | `astro_time_t` | < The time of the eclipse at its peak. |
| `sd_penum` | `f64` | < The semi-duration of the penumbral phase in minutes. |
| `sd_partial` | `f64` | < The semi-duration of the partial phase in minutes, or 0.0 if none. |
| `sd_total` | `f64` | < The semi-duration of the total phase in minutes, or 0.0 if none. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_lunar_eclipse_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_global_solar_eclipse_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Reports the time and geographic location of the peak of a solar eclipse.

Returned by #Astronomy_SearchGlobalSolarEclipse or #Astronomy_NextGlobalSolarEclipse
to report information about a solar eclipse event.
If a solar eclipse is found, `status` holds `ASTRO_SUCCESS` and `kind`, `peak`, and `distance`
have valid values. The `latitude` and `longitude` are set only for total and annular eclipses
(see more below).
If `status` holds any value other than `ASTRO_SUCCESS`, it is an error code;
in that case, `kind` holds `ECLIPSE_NONE` and all the other fields are undefined.

The eclipse is classified as partial, annular, or total, depending on the
maximum amount of the Sun's disc obscured, as seen at the peak location
on the surface of the Earth.

The `kind` field thus holds `ECLIPSE_PARTIAL`, `ECLIPSE_ANNULAR`, or `ECLIPSE_TOTAL`.
A total eclipse is when the peak observer sees the Sun completely blocked by the Moon.
An annular eclipse is like a total eclipse, but the Moon is too far from the Earth's surface
to completely block the Sun; instead, the Sun takes on a ring-shaped appearance.
A partial eclipse is when the Moon blocks part of the Sun's disc, but nobody on the Earth
observes either a total or annular eclipse.

If `kind` is `ECLIPSE_TOTAL` or `ECLIPSE_ANNULAR`, the `latitude` and `longitude`
fields give the geographic coordinates of the center of the Moon's shadow projected
onto the daytime side of the Earth at the instant of the eclipse's peak.
If `kind` has any other value, `latitude` and `longitude` are undefined and should
not be used.

For total or annular eclipses, the `obscuration` field holds the fraction (0, 1]
of the Sun's apparent disc area that is blocked from view by the Moon's silhouette,
as seen by an observer located at the geographic coordinates `latitude`, `longitude`
at the darkest time `peak`. The value will always be 1 for total eclipses, and less than
1 for annular eclipses.
For partial eclipses, `obscuration` is undefined and should not be used.
This is because there is little practical use for an obscuration value of
a partial eclipse without supplying a particular observation location.
Developers who wish to find an obscuration value for partial solar eclipses should therefore use
#Astronomy_SearchLocalSolarEclipse and provide the geographic coordinates of an observer.

```rust
pub struct astro_global_solar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub obscuration: f64,
    pub peak: astro_time_t,
    pub distance: f64,
    pub latitude: f64,
    pub longitude: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `kind` | `astro_eclipse_kind_t` | < The type of solar eclipse found. |
| `obscuration` | `f64` | < The peak fraction of the Sun's apparent disc area obscured by the Moon (total and annular eclipses only). |
| `peak` | `astro_time_t` | < The date and time when the solar eclipse is darkest. This is the instant when the axis of the Moon's shadow cone passes closest to the Earth's center. |
| `distance` | `f64` | < The distance between the Sun/Moon shadow axis and the center of the Earth, in kilometers. |
| `latitude` | `f64` | < The geographic latitude at the center of the peak eclipse shadow. |
| `longitude` | `f64` | < The geographic longitude at the center of the peak eclipse shadow. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_global_solar_eclipse_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_eclipse_event_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Holds a time and the observed altitude of the Sun at that time.

When reporting a solar eclipse observed at a specific location on the Earth
(a "local" solar eclipse), a series of events occur. In addition
to the time of each event, it is important to know the altitude of the Sun,
because each event may be invisible to the observer if the Sun is below
the horizon.

If `altitude` is negative, the event is theoretical only; it would be
visible if the Earth were transparent, but the observer cannot actually see it.
If `altitude` is positive but less than a few degrees, visibility will be impaired by
atmospheric interference (sunrise or sunset conditions).

```rust
pub struct astro_eclipse_event_t {
    pub time: astro_time_t,
    pub altitude: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `time` | `astro_time_t` | < The date and time of the event. |
| `altitude` | `f64` | < The angular altitude of the center of the Sun above/below the horizon, at `time`, corrected for atmospheric refraction and expressed in degrees. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_eclipse_event_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_local_solar_eclipse_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about a solar eclipse as seen by an observer at a given time and geographic location.

Returned by #Astronomy_SearchLocalSolarEclipse or #Astronomy_NextLocalSolarEclipse
to report information about a solar eclipse as seen at a given geographic location.
If a solar eclipse is found, `status` holds `ASTRO_SUCCESS` and the other fields are set.
If `status` holds any other value, it is an error code and the other fields are undefined.

When a solar eclipse is found, it is classified as partial, annular, or total.
The `kind` field thus holds `ECLIPSE_PARTIAL`, `ECLIPSE_ANNULAR`, or `ECLIPSE_TOTAL`.
A partial solar eclipse is when the Moon does not line up directly enough with the Sun
to completely block the Sun's light from reaching the observer.
An annular eclipse occurs when the Moon's disc is completely visible against the Sun
but the Moon is too far away to completely block the Sun's light; this leaves the
Sun with a ring-like appearance.
A total eclipse occurs when the Moon is close enough to the Earth and aligned with the
Sun just right to completely block all sunlight from reaching the observer.

The `obscuration` field reports what fraction of the Sun's disc appears blocked
by the Moon when viewed by the observer at the peak eclipse time.
This is a value that ranges from 0 (no blockage) to 1 (total eclipse).
The obscuration value will be between 0 and 1 for partial eclipses and annular eclipses.
The value will be exactly 1 for total eclipses. Obscuration gives an indication
of how dark the eclipse appears.

There are 5 "event" fields, each of which contains a time and a solar altitude.
Field `peak` holds the date and time of the center of the eclipse, when it is at its peak.
The fields `partial_begin` and `partial_end` are always set, and indicate when
the eclipse begins/ends. If the eclipse reaches totality or becomes annular,
`total_begin` and `total_end` indicate when the total/annular phase begins/ends.
When an event field is valid, the caller must also check its `altitude` field to
see whether the Sun is above the horizon at that time. See #astro_eclipse_kind_t
for more information.

```rust
pub struct astro_local_solar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub obscuration: f64,
    pub partial_begin: astro_eclipse_event_t,
    pub total_begin: astro_eclipse_event_t,
    pub peak: astro_eclipse_event_t,
    pub total_end: astro_eclipse_event_t,
    pub partial_end: astro_eclipse_event_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `kind` | `astro_eclipse_kind_t` | < The type of solar eclipse found: `ECLIPSE_PARTIAL`, `ECLIPSE_ANNULAR`, or `ECLIPSE_TOTAL`. |
| `obscuration` | `f64` | < The fraction of the Sun's apparent disc area obscured by the Moon at the eclipse peak. |
| `partial_begin` | `astro_eclipse_event_t` | < The time and Sun altitude at the beginning of the eclipse. |
| `total_begin` | `astro_eclipse_event_t` | < If this is an annular or a total eclipse, the time and Sun altitude when annular/total phase begins; otherwise invalid. |
| `peak` | `astro_eclipse_event_t` | < The time and Sun altitude when the eclipse reaches its peak. |
| `total_end` | `astro_eclipse_event_t` | < If this is an annular or a total eclipse, the time and Sun altitude when annular/total phase ends; otherwise invalid. |
| `partial_end` | `astro_eclipse_event_t` | < The time and Sun altitude at the end of the eclipse. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_local_solar_eclipse_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_transit_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about a transit of Mercury or Venus, as seen from the Earth.

Returned by #Astronomy_SearchTransit or #Astronomy_NextTransit to report
information about a transit of Mercury or Venus.
A transit is when Mercury or Venus passes between the Sun and Earth so that
the other planet is seen in silhouette against the Sun.

The `start` field reports the moment in time when the planet first becomes
visible against the Sun in its background.
The `peak` field reports when the planet is most aligned with the Sun,
as seen from the Earth.
The `finish` field reports the last moment when the planet is visible
against the Sun in its background.

The calculations are performed from the point of view of a geocentric observer.

```rust
pub struct astro_transit_t {
    pub status: astro_status_t,
    pub start: astro_time_t,
    pub peak: astro_time_t,
    pub finish: astro_time_t,
    pub separation: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `start` | `astro_time_t` | < Date and time at the beginning of the transit. |
| `peak` | `astro_time_t` | < Date and time of the peak of the transit. |
| `finish` | `astro_time_t` | < Date and time at the end of the transit. |
| `separation` | `f64` | < Angular separation in arcminutes between the centers of the Sun and the planet at time `peak`. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_transit_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_aberration_t`

@brief   Aberration calculation options.

[Aberration](https://en.wikipedia.org/wiki/Aberration_of_light) is an effect
causing the apparent direction of an observed body to be shifted due to transverse
movement of the Earth with respect to the rays of light coming from that body.
This angular correction can be anywhere from 0 to about 20 arcseconds,
depending on the position of the observed body relative to the instantaneous
velocity vector of the Earth.

Some Astronomy Engine functions allow optional correction for aberration by
passing in a value of this enumerated type.

Aberration correction is useful to improve accuracy of coordinates of
apparent locations of bodies seen from the Earth.
However, because aberration affects not only the observed body (such as a planet)
but the surrounding stars, aberration may be unhelpful (for example)
for determining exactly when a planet crosses from one constellation to another.

```rust
pub type astro_aberration_t = ::std::os::raw::c_uint;
```

### Type Alias `astro_equator_date_t`

@brief   Selects the date for which the Earth's equator is to be used for representing equatorial coordinates.

The Earth's equator is not always in the same plane due to precession and nutation.

Sometimes it is useful to have a fixed plane of reference for equatorial coordinates
across different calendar dates.  In these cases, a fixed *epoch*, or reference time,
is helpful. Astronomy Engine provides the J2000 epoch for such cases.  This refers
to the plane of the Earth's orbit as it was on noon UTC on 1 January 2000.

For some other purposes, it is more helpful to represent coordinates using the Earth's
equator exactly as it is on that date. For example, when calculating rise/set times
or horizontal coordinates, it is most accurate to use the orientation of the Earth's
equator at that same date and time. For these uses, Astronomy Engine allows *of-date*
calculations.

```rust
pub type astro_equator_date_t = ::std::os::raw::c_uint;
```

### Type Alias `astro_direction_t`

@brief Selects whether to search for a rise time or a set time.

The #Astronomy_SearchRiseSetEx function finds the rise or set time of a body
depending on the value of its `direction` parameter.

```rust
pub type astro_direction_t = ::std::os::raw::c_int;
```

### Struct `astro_constellation_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Reports the constellation that a given celestial point lies within.

The #Astronomy_Constellation function returns this struct
to report which constellation corresponds with a given point in the sky.
Constellations are defined with respect to the B1875 equatorial system
per IAU standard. Although `Astronomy.Constellation` requires J2000 equatorial
coordinates, the struct contains converted B1875 coordinates for reference.

```rust
pub struct astro_constellation_t {
    pub status: astro_status_t,
    pub symbol: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub ra_1875: f64,
    pub dec_1875: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `symbol` | `*const ::std::os::raw::c_char` | < 3-character mnemonic symbol for the constellation, e.g. "Ori". |
| `name` | `*const ::std::os::raw::c_char` | < Full name of constellation, e.g. "Orion". |
| `ra_1875` | `f64` | < Right ascension expressed in B1875 coordinates. |
| `dec_1875` | `f64` | < Declination expressed in B1875 coordinates. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_constellation_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_time_format_t`

@brief Selects the output format of the function #Astronomy_FormatTime.

```rust
pub type astro_time_format_t = ::std::os::raw::c_uint;
```

### Struct `astro_libration_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Lunar libration angles, returned by #Astronomy_Libration.

```rust
pub struct astro_libration_t {
    pub elat: f64,
    pub elon: f64,
    pub mlat: f64,
    pub mlon: f64,
    pub dist_km: f64,
    pub diam_deg: f64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `elat` | `f64` | < Sub-Earth libration ecliptic latitude angle, in degrees. |
| `elon` | `f64` | < Sub-Earth libration ecliptic longitude angle, in degrees. |
| `mlat` | `f64` | < Moon's geocentric ecliptic latitude, in degrees. |
| `mlon` | `f64` | < Moon's geocentric ecliptic longitude, in degrees. |
| `dist_km` | `f64` | < Distance between the centers of the Earth and Moon in kilometers. |
| `diam_deg` | `f64` | < The apparent angular diameter of the Moon, in degrees, as seen from the center of the Earth. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_libration_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_axis_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about a body's rotation axis at a given time.

This structure is returned by #Astronomy_RotationAxis to report
the orientation of a body's rotation axis at a given moment in time.
The axis is specified by the direction in space that the body's north pole
points, using angular equatorial coordinates in the J2000 system (EQJ).

Thus `ra` is the right ascension, and `dec` is the declination, of the
body's north pole vector at the given moment in time. The north pole
of a body is defined as the pole that lies on the north side of the
[Solar System's invariable plane](https://en.wikipedia.org/wiki/Invariable_plane),
regardless of the body's direction of rotation.

The `spin` field indicates the angular position of a prime meridian
arbitrarily recommended for the body by the International Astronomical
Union (IAU).

The fields `ra`, `dec`, and `spin` correspond to the variables
α0, δ0, and W, respectively, from
[Report of the IAU Working Group on Cartographic Coordinates and Rotational Elements: 2015](https://astropedia.astrogeology.usgs.gov/download/Docs/WGCCRE/WGCCRE2015reprint.pdf).

The field `north` is a unit vector pointing in the direction of the body's north pole.
It is expressed in the equatorial J2000 system (EQJ).

```rust
pub struct astro_axis_t {
    pub status: astro_status_t,
    pub ra: f64,
    pub dec: f64,
    pub spin: f64,
    pub north: astro_vector_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `ra` | `f64` | < The J2000 right ascension of the body's north pole direction, in sidereal hours. |
| `dec` | `f64` | < The J2000 declination of the body's north pole direction, in degrees. |
| `spin` | `f64` | < Rotation angle of the body's prime meridian, in degrees. |
| `north` | `astro_vector_t` | < A J2000 dimensionless unit vector pointing in the direction of the body's north pole. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_axis_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_jupiter_moons_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Holds the positions and velocities of Jupiter's major 4 moons.

The #Astronomy_JupiterMoons function returns this struct
to report position and velocity vectors for Jupiter's largest 4 moons
Io, Europa, Ganymede, and Callisto. Each position vector is relative
to the center of Jupiter. Both position and velocity are oriented in
the EQJ system (that is, using Earth's equator at the J2000 epoch.)
The positions are expressed in astronomical units (AU),
and the velocities in AU/day.

```rust
pub struct astro_jupiter_moons_t {
    pub io: astro_state_vector_t,
    pub europa: astro_state_vector_t,
    pub ganymede: astro_state_vector_t,
    pub callisto: astro_state_vector_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `io` | `astro_state_vector_t` | < Jovicentric position and velocity of Io. |
| `europa` | `astro_state_vector_t` | < Jovicentric position and velocity of Europa. |
| `ganymede` | `astro_state_vector_t` | < Jovicentric position and velocity of Ganymede. |
| `callisto` | `astro_state_vector_t` | < Jovicentric position and velocity of Callisto. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_jupiter_moons_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_node_kind_t`

@brief  Indicates whether a crossing through the ecliptic plane is ascending or descending.

```rust
pub type astro_node_kind_t = ::std::os::raw::c_int;
```

### Struct `astro_node_event_t`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

@brief Information about an ascending or descending node of a body.

This structure is returned by #Astronomy_SearchMoonNode and #Astronomy_NextMoonNode
to report information about the center of the Moon passing through the ecliptic plane.

```rust
pub struct astro_node_event_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub kind: astro_node_kind_t,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `astro_status_t` | < `ASTRO_SUCCESS` if this struct is valid; otherwise an error code. |
| `time` | `astro_time_t` | < The time when the body passes through the ecliptic plane. |
| `kind` | `astro_node_kind_t` | < Either `ASCENDING_NODE` or `DESCENDING_NODE`, depending on the direction of the ecliptic plane crossing. |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_node_event_t { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Struct `astro_grav_sim_s`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: None, int: None })`

```rust
pub struct astro_grav_sim_s {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> astro_grav_sim_s { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Type Alias `astro_grav_sim_t`

@brief A data type used for managing simulation of the gravitational forces on a small body.

This is an opaque data type used to hold the internal state of
a numeric integrator used to calculate the trajectory of a small
body moving through the Solar System.

```rust
pub type astro_grav_sim_t = astro_grav_sim_s;
```

### Type Alias `astro_position_func_t`

@brief A function for which to solve a light-travel time problem.

The function #Astronomy_CorrectLightTravel solves a generalized
problem of deducing how far in the past light must have left
a target object to be seen by an observer at a specified time.
This function pointer type expresses an arbitrary position vector
as function of time. Such a function must be passed to
`Astronomy_CorrectLightTravel`.

```rust
pub type astro_position_func_t = ::std::option::Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void, astro_time_t) -> astro_vector_t>;
```

## Functions

### Function `Astronomy_DeltaT_EspenakMeeus`

```rust
pub unsafe extern "C" fn Astronomy_DeltaT_EspenakMeeus(ut: f64) -> f64;
```

### Function `Astronomy_DeltaT_JplHorizons`

```rust
pub unsafe extern "C" fn Astronomy_DeltaT_JplHorizons(ut: f64) -> f64;
```

### Function `Astronomy_SetDeltaTFunction`

```rust
pub unsafe extern "C" fn Astronomy_SetDeltaTFunction(func: astro_deltat_func);
```

### Function `Astronomy_Reset`

```rust
pub unsafe extern "C" fn Astronomy_Reset();
```

### Function `Astronomy_VectorLength`

```rust
pub unsafe extern "C" fn Astronomy_VectorLength(vector: astro_vector_t) -> f64;
```

### Function `Astronomy_AngleBetween`

```rust
pub unsafe extern "C" fn Astronomy_AngleBetween(a: astro_vector_t, b: astro_vector_t) -> astro_angle_result_t;
```

### Function `Astronomy_BodyName`

```rust
pub unsafe extern "C" fn Astronomy_BodyName(body: astro_body_t) -> *const ::std::os::raw::c_char;
```

### Function `Astronomy_BodyCode`

```rust
pub unsafe extern "C" fn Astronomy_BodyCode(name: *const ::std::os::raw::c_char) -> astro_body_t;
```

### Function `Astronomy_MakeObserver`

```rust
pub unsafe extern "C" fn Astronomy_MakeObserver(latitude: f64, longitude: f64, height: f64) -> astro_observer_t;
```

### Function `Astronomy_CurrentTime`

```rust
pub unsafe extern "C" fn Astronomy_CurrentTime() -> astro_time_t;
```

### Function `Astronomy_MakeTime`

```rust
pub unsafe extern "C" fn Astronomy_MakeTime(year: ::std::os::raw::c_int, month: ::std::os::raw::c_int, day: ::std::os::raw::c_int, hour: ::std::os::raw::c_int, minute: ::std::os::raw::c_int, second: f64) -> astro_time_t;
```

### Function `Astronomy_TimeFromUtc`

```rust
pub unsafe extern "C" fn Astronomy_TimeFromUtc(utc: astro_utc_t) -> astro_time_t;
```

### Function `Astronomy_UtcFromTime`

```rust
pub unsafe extern "C" fn Astronomy_UtcFromTime(time: astro_time_t) -> astro_utc_t;
```

### Function `Astronomy_FormatTime`

```rust
pub unsafe extern "C" fn Astronomy_FormatTime(time: astro_time_t, format: astro_time_format_t, text: *mut ::std::os::raw::c_char, size: usize) -> astro_status_t;
```

### Function `Astronomy_TimeFromDays`

```rust
pub unsafe extern "C" fn Astronomy_TimeFromDays(ut: f64) -> astro_time_t;
```

### Function `Astronomy_TerrestrialTime`

```rust
pub unsafe extern "C" fn Astronomy_TerrestrialTime(tt: f64) -> astro_time_t;
```

### Function `Astronomy_AddDays`

```rust
pub unsafe extern "C" fn Astronomy_AddDays(time: astro_time_t, days: f64) -> astro_time_t;
```

### Function `Astronomy_SiderealTime`

```rust
pub unsafe extern "C" fn Astronomy_SiderealTime(time: *mut astro_time_t) -> f64;
```

### Function `Astronomy_HelioDistance`

```rust
pub unsafe extern "C" fn Astronomy_HelioDistance(body: astro_body_t, time: astro_time_t) -> astro_func_result_t;
```

### Function `Astronomy_HelioVector`

```rust
pub unsafe extern "C" fn Astronomy_HelioVector(body: astro_body_t, time: astro_time_t) -> astro_vector_t;
```

### Function `Astronomy_GeoVector`

```rust
pub unsafe extern "C" fn Astronomy_GeoVector(body: astro_body_t, time: astro_time_t, aberration: astro_aberration_t) -> astro_vector_t;
```

### Function `Astronomy_GeoMoon`

```rust
pub unsafe extern "C" fn Astronomy_GeoMoon(time: astro_time_t) -> astro_vector_t;
```

### Function `Astronomy_EclipticGeoMoon`

```rust
pub unsafe extern "C" fn Astronomy_EclipticGeoMoon(time: astro_time_t) -> astro_spherical_t;
```

### Function `Astronomy_GeoMoonState`

```rust
pub unsafe extern "C" fn Astronomy_GeoMoonState(time: astro_time_t) -> astro_state_vector_t;
```

### Function `Astronomy_GeoEmbState`

```rust
pub unsafe extern "C" fn Astronomy_GeoEmbState(time: astro_time_t) -> astro_state_vector_t;
```

### Function `Astronomy_Libration`

```rust
pub unsafe extern "C" fn Astronomy_Libration(time: astro_time_t) -> astro_libration_t;
```

### Function `Astronomy_BaryState`

```rust
pub unsafe extern "C" fn Astronomy_BaryState(body: astro_body_t, time: astro_time_t) -> astro_state_vector_t;
```

### Function `Astronomy_HelioState`

```rust
pub unsafe extern "C" fn Astronomy_HelioState(body: astro_body_t, time: astro_time_t) -> astro_state_vector_t;
```

### Function `Astronomy_MassProduct`

```rust
pub unsafe extern "C" fn Astronomy_MassProduct(body: astro_body_t) -> f64;
```

### Function `Astronomy_PlanetOrbitalPeriod`

```rust
pub unsafe extern "C" fn Astronomy_PlanetOrbitalPeriod(body: astro_body_t) -> f64;
```

### Function `Astronomy_LagrangePoint`

```rust
pub unsafe extern "C" fn Astronomy_LagrangePoint(point: ::std::os::raw::c_int, time: astro_time_t, major_body: astro_body_t, minor_body: astro_body_t) -> astro_state_vector_t;
```

### Function `Astronomy_LagrangePointFast`

```rust
pub unsafe extern "C" fn Astronomy_LagrangePointFast(point: ::std::os::raw::c_int, major_state: astro_state_vector_t, major_mass: f64, minor_state: astro_state_vector_t, minor_mass: f64) -> astro_state_vector_t;
```

### Function `Astronomy_JupiterMoons`

```rust
pub unsafe extern "C" fn Astronomy_JupiterMoons(time: astro_time_t) -> astro_jupiter_moons_t;
```

### Function `Astronomy_Equator`

```rust
pub unsafe extern "C" fn Astronomy_Equator(body: astro_body_t, time: *mut astro_time_t, observer: astro_observer_t, equdate: astro_equator_date_t, aberration: astro_aberration_t) -> astro_equatorial_t;
```

### Function `Astronomy_ObserverVector`

```rust
pub unsafe extern "C" fn Astronomy_ObserverVector(time: *mut astro_time_t, observer: astro_observer_t, equdate: astro_equator_date_t) -> astro_vector_t;
```

### Function `Astronomy_ObserverState`

```rust
pub unsafe extern "C" fn Astronomy_ObserverState(time: *mut astro_time_t, observer: astro_observer_t, equdate: astro_equator_date_t) -> astro_state_vector_t;
```

### Function `Astronomy_VectorObserver`

```rust
pub unsafe extern "C" fn Astronomy_VectorObserver(vector: *mut astro_vector_t, equdate: astro_equator_date_t) -> astro_observer_t;
```

### Function `Astronomy_ObserverGravity`

```rust
pub unsafe extern "C" fn Astronomy_ObserverGravity(latitude: f64, height: f64) -> f64;
```

### Function `Astronomy_SunPosition`

```rust
pub unsafe extern "C" fn Astronomy_SunPosition(time: astro_time_t) -> astro_ecliptic_t;
```

### Function `Astronomy_Ecliptic`

```rust
pub unsafe extern "C" fn Astronomy_Ecliptic(eqj: astro_vector_t) -> astro_ecliptic_t;
```

### Function `Astronomy_EclipticLongitude`

```rust
pub unsafe extern "C" fn Astronomy_EclipticLongitude(body: astro_body_t, time: astro_time_t) -> astro_angle_result_t;
```

### Function `Astronomy_Horizon`

```rust
pub unsafe extern "C" fn Astronomy_Horizon(time: *mut astro_time_t, observer: astro_observer_t, ra: f64, dec: f64, refraction: astro_refraction_t) -> astro_horizon_t;
```

### Function `Astronomy_AngleFromSun`

```rust
pub unsafe extern "C" fn Astronomy_AngleFromSun(body: astro_body_t, time: astro_time_t) -> astro_angle_result_t;
```

### Function `Astronomy_Elongation`

```rust
pub unsafe extern "C" fn Astronomy_Elongation(body: astro_body_t, time: astro_time_t) -> astro_elongation_t;
```

### Function `Astronomy_SearchMaxElongation`

```rust
pub unsafe extern "C" fn Astronomy_SearchMaxElongation(body: astro_body_t, startTime: astro_time_t) -> astro_elongation_t;
```

### Function `Astronomy_PairLongitude`

```rust
pub unsafe extern "C" fn Astronomy_PairLongitude(body1: astro_body_t, body2: astro_body_t, time: astro_time_t) -> astro_angle_result_t;
```

### Function `Astronomy_SearchRelativeLongitude`

@endcond

```rust
pub unsafe extern "C" fn Astronomy_SearchRelativeLongitude(body: astro_body_t, targetRelLon: f64, startTime: astro_time_t) -> astro_search_result_t;
```

### Function `Astronomy_MoonPhase`

```rust
pub unsafe extern "C" fn Astronomy_MoonPhase(time: astro_time_t) -> astro_angle_result_t;
```

### Function `Astronomy_SearchMoonPhase`

```rust
pub unsafe extern "C" fn Astronomy_SearchMoonPhase(targetLon: f64, startTime: astro_time_t, limitDays: f64) -> astro_search_result_t;
```

### Function `Astronomy_SearchMoonQuarter`

```rust
pub unsafe extern "C" fn Astronomy_SearchMoonQuarter(startTime: astro_time_t) -> astro_moon_quarter_t;
```

### Function `Astronomy_NextMoonQuarter`

```rust
pub unsafe extern "C" fn Astronomy_NextMoonQuarter(mq: astro_moon_quarter_t) -> astro_moon_quarter_t;
```

### Function `Astronomy_SearchLunarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_SearchLunarEclipse(startTime: astro_time_t) -> astro_lunar_eclipse_t;
```

### Function `Astronomy_NextLunarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_NextLunarEclipse(prevEclipseTime: astro_time_t) -> astro_lunar_eclipse_t;
```

### Function `Astronomy_SearchGlobalSolarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_SearchGlobalSolarEclipse(startTime: astro_time_t) -> astro_global_solar_eclipse_t;
```

### Function `Astronomy_NextGlobalSolarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_NextGlobalSolarEclipse(prevEclipseTime: astro_time_t) -> astro_global_solar_eclipse_t;
```

### Function `Astronomy_SearchLocalSolarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_SearchLocalSolarEclipse(startTime: astro_time_t, observer: astro_observer_t) -> astro_local_solar_eclipse_t;
```

### Function `Astronomy_NextLocalSolarEclipse`

```rust
pub unsafe extern "C" fn Astronomy_NextLocalSolarEclipse(prevEclipseTime: astro_time_t, observer: astro_observer_t) -> astro_local_solar_eclipse_t;
```

### Function `Astronomy_SearchTransit`

```rust
pub unsafe extern "C" fn Astronomy_SearchTransit(body: astro_body_t, startTime: astro_time_t) -> astro_transit_t;
```

### Function `Astronomy_NextTransit`

```rust
pub unsafe extern "C" fn Astronomy_NextTransit(body: astro_body_t, prevTransitTime: astro_time_t) -> astro_transit_t;
```

### Function `Astronomy_SearchMoonNode`

```rust
pub unsafe extern "C" fn Astronomy_SearchMoonNode(startTime: astro_time_t) -> astro_node_event_t;
```

### Function `Astronomy_NextMoonNode`

```rust
pub unsafe extern "C" fn Astronomy_NextMoonNode(prevNode: astro_node_event_t) -> astro_node_event_t;
```

### Function `Astronomy_Search`

```rust
pub unsafe extern "C" fn Astronomy_Search(func: astro_search_func_t, context: *mut ::std::os::raw::c_void, t1: astro_time_t, t2: astro_time_t, dt_tolerance_seconds: f64) -> astro_search_result_t;
```

### Function `Astronomy_SearchSunLongitude`

```rust
pub unsafe extern "C" fn Astronomy_SearchSunLongitude(targetLon: f64, startTime: astro_time_t, limitDays: f64) -> astro_search_result_t;
```

### Function `Astronomy_SearchHourAngleEx`

```rust
pub unsafe extern "C" fn Astronomy_SearchHourAngleEx(body: astro_body_t, observer: astro_observer_t, hourAngle: f64, startTime: astro_time_t, direction: ::std::os::raw::c_int) -> astro_hour_angle_t;
```

### Function `Astronomy_HourAngle`

```rust
pub unsafe extern "C" fn Astronomy_HourAngle(body: astro_body_t, time: *mut astro_time_t, observer: astro_observer_t) -> astro_func_result_t;
```

### Function `Astronomy_SearchRiseSetEx`

@endcond

```rust
pub unsafe extern "C" fn Astronomy_SearchRiseSetEx(body: astro_body_t, observer: astro_observer_t, direction: astro_direction_t, startTime: astro_time_t, limitDays: f64, metersAboveGround: f64) -> astro_search_result_t;
```

### Function `Astronomy_SearchAltitude`

```rust
pub unsafe extern "C" fn Astronomy_SearchAltitude(body: astro_body_t, observer: astro_observer_t, direction: astro_direction_t, startTime: astro_time_t, limitDays: f64, altitude: f64) -> astro_search_result_t;
```

### Function `Astronomy_Atmosphere`

```rust
pub unsafe extern "C" fn Astronomy_Atmosphere(elevationMeters: f64) -> astro_atmosphere_t;
```

### Function `Astronomy_RotationAxis`

```rust
pub unsafe extern "C" fn Astronomy_RotationAxis(body: astro_body_t, time: *mut astro_time_t) -> astro_axis_t;
```

### Function `Astronomy_Seasons`

```rust
pub unsafe extern "C" fn Astronomy_Seasons(year: ::std::os::raw::c_int) -> astro_seasons_t;
```

### Function `Astronomy_Illumination`

```rust
pub unsafe extern "C" fn Astronomy_Illumination(body: astro_body_t, time: astro_time_t) -> astro_illum_t;
```

### Function `Astronomy_SearchPeakMagnitude`

```rust
pub unsafe extern "C" fn Astronomy_SearchPeakMagnitude(body: astro_body_t, startTime: astro_time_t) -> astro_illum_t;
```

### Function `Astronomy_SearchLunarApsis`

```rust
pub unsafe extern "C" fn Astronomy_SearchLunarApsis(startTime: astro_time_t) -> astro_apsis_t;
```

### Function `Astronomy_NextLunarApsis`

```rust
pub unsafe extern "C" fn Astronomy_NextLunarApsis(apsis: astro_apsis_t) -> astro_apsis_t;
```

### Function `Astronomy_SearchPlanetApsis`

```rust
pub unsafe extern "C" fn Astronomy_SearchPlanetApsis(body: astro_body_t, startTime: astro_time_t) -> astro_apsis_t;
```

### Function `Astronomy_NextPlanetApsis`

```rust
pub unsafe extern "C" fn Astronomy_NextPlanetApsis(body: astro_body_t, apsis: astro_apsis_t) -> astro_apsis_t;
```

### Function `Astronomy_IdentityMatrix`

```rust
pub unsafe extern "C" fn Astronomy_IdentityMatrix() -> astro_rotation_t;
```

### Function `Astronomy_InverseRotation`

```rust
pub unsafe extern "C" fn Astronomy_InverseRotation(rotation: astro_rotation_t) -> astro_rotation_t;
```

### Function `Astronomy_CombineRotation`

```rust
pub unsafe extern "C" fn Astronomy_CombineRotation(a: astro_rotation_t, b: astro_rotation_t) -> astro_rotation_t;
```

### Function `Astronomy_Pivot`

```rust
pub unsafe extern "C" fn Astronomy_Pivot(rotation: astro_rotation_t, axis: ::std::os::raw::c_int, angle: f64) -> astro_rotation_t;
```

### Function `Astronomy_VectorFromSphere`

```rust
pub unsafe extern "C" fn Astronomy_VectorFromSphere(sphere: astro_spherical_t, time: astro_time_t) -> astro_vector_t;
```

### Function `Astronomy_SphereFromVector`

```rust
pub unsafe extern "C" fn Astronomy_SphereFromVector(vector: astro_vector_t) -> astro_spherical_t;
```

### Function `Astronomy_EquatorFromVector`

```rust
pub unsafe extern "C" fn Astronomy_EquatorFromVector(vector: astro_vector_t) -> astro_equatorial_t;
```

### Function `Astronomy_VectorFromHorizon`

```rust
pub unsafe extern "C" fn Astronomy_VectorFromHorizon(sphere: astro_spherical_t, time: astro_time_t, refraction: astro_refraction_t) -> astro_vector_t;
```

### Function `Astronomy_HorizonFromVector`

```rust
pub unsafe extern "C" fn Astronomy_HorizonFromVector(vector: astro_vector_t, refraction: astro_refraction_t) -> astro_spherical_t;
```

### Function `Astronomy_RotateVector`

```rust
pub unsafe extern "C" fn Astronomy_RotateVector(rotation: astro_rotation_t, vector: astro_vector_t) -> astro_vector_t;
```

### Function `Astronomy_RotateState`

```rust
pub unsafe extern "C" fn Astronomy_RotateState(rotation: astro_rotation_t, state: astro_state_vector_t) -> astro_state_vector_t;
```

### Function `Astronomy_Rotation_EQD_EQJ`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQD_EQJ(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQD_ECL`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQD_ECL(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQD_ECT`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQD_ECT(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQD_HOR`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQD_HOR(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQJ_EQD`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQJ_EQD(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQJ_ECT`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQJ_ECT(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQJ_ECL`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQJ_ECL() -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQJ_HOR`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQJ_HOR(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_ECL_EQD`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_ECL_EQD(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_ECL_EQJ`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_ECL_EQJ() -> astro_rotation_t;
```

### Function `Astronomy_Rotation_ECL_HOR`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_ECL_HOR(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_ECT_EQJ`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_ECT_EQJ(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_ECT_EQD`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_ECT_EQD(time: *mut astro_time_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_HOR_EQD`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_HOR_EQD(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_HOR_EQJ`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_HOR_EQJ(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_HOR_ECL`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_HOR_ECL(time: *mut astro_time_t, observer: astro_observer_t) -> astro_rotation_t;
```

### Function `Astronomy_Rotation_EQJ_GAL`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_EQJ_GAL() -> astro_rotation_t;
```

### Function `Astronomy_Rotation_GAL_EQJ`

```rust
pub unsafe extern "C" fn Astronomy_Rotation_GAL_EQJ() -> astro_rotation_t;
```

### Function `Astronomy_Refraction`

```rust
pub unsafe extern "C" fn Astronomy_Refraction(refraction: astro_refraction_t, altitude: f64) -> f64;
```

### Function `Astronomy_InverseRefraction`

```rust
pub unsafe extern "C" fn Astronomy_InverseRefraction(refraction: astro_refraction_t, bent_altitude: f64) -> f64;
```

### Function `Astronomy_Constellation`

```rust
pub unsafe extern "C" fn Astronomy_Constellation(ra: f64, dec: f64) -> astro_constellation_t;
```

### Function `Astronomy_GravSimInit`

```rust
pub unsafe extern "C" fn Astronomy_GravSimInit(simOut: *mut *mut astro_grav_sim_t, originBody: astro_body_t, time: astro_time_t, numBodies: ::std::os::raw::c_int, bodyStateArray: *const astro_state_vector_t) -> astro_status_t;
```

### Function `Astronomy_GravSimUpdate`

```rust
pub unsafe extern "C" fn Astronomy_GravSimUpdate(sim: *mut astro_grav_sim_t, time: astro_time_t, numBodies: ::std::os::raw::c_int, bodyStateArray: *mut astro_state_vector_t) -> astro_status_t;
```

### Function `Astronomy_GravSimBodyState`

```rust
pub unsafe extern "C" fn Astronomy_GravSimBodyState(sim: *mut astro_grav_sim_t, body: astro_body_t) -> astro_state_vector_t;
```

### Function `Astronomy_GravSimTime`

```rust
pub unsafe extern "C" fn Astronomy_GravSimTime(sim: *const astro_grav_sim_t) -> astro_time_t;
```

### Function `Astronomy_GravSimNumBodies`

```rust
pub unsafe extern "C" fn Astronomy_GravSimNumBodies(sim: *const astro_grav_sim_t) -> ::std::os::raw::c_int;
```

### Function `Astronomy_GravSimOrigin`

```rust
pub unsafe extern "C" fn Astronomy_GravSimOrigin(sim: *const astro_grav_sim_t) -> astro_body_t;
```

### Function `Astronomy_GravSimSwap`

```rust
pub unsafe extern "C" fn Astronomy_GravSimSwap(sim: *mut astro_grav_sim_t);
```

### Function `Astronomy_GravSimFree`

```rust
pub unsafe extern "C" fn Astronomy_GravSimFree(sim: *mut astro_grav_sim_t);
```

### Function `Astronomy_CorrectLightTravel`

```rust
pub unsafe extern "C" fn Astronomy_CorrectLightTravel(context: *mut ::std::os::raw::c_void, func: astro_position_func_t, time: astro_time_t) -> astro_vector_t;
```

### Function `Astronomy_BackdatePosition`

```rust
pub unsafe extern "C" fn Astronomy_BackdatePosition(time: astro_time_t, observerBody: astro_body_t, targetBody: astro_body_t, aberration: astro_aberration_t) -> astro_vector_t;
```

### Function `Astronomy_DefineStar`

```rust
pub unsafe extern "C" fn Astronomy_DefineStar(body: astro_body_t, ra: f64, dec: f64, distanceLightYears: f64) -> astro_status_t;
```

## Constants and Statics

### Constant `C_AUDAY`

```rust
pub const C_AUDAY: f64 = 173.1446326846693;
```

### Constant `AU_PER_LY`

```rust
pub const AU_PER_LY: f64 = 63241.07708807546;
```

### Constant `DEG2RAD`

```rust
pub const DEG2RAD: f64 = 0.017453292519943295;
```

### Constant `HOUR2RAD`

```rust
pub const HOUR2RAD: f64 = 0.26179938779914946;
```

### Constant `RAD2DEG`

```rust
pub const RAD2DEG: f64 = 57.29577951308232;
```

### Constant `RAD2HOUR`

```rust
pub const RAD2HOUR: f64 = 3.819718634205488;
```

### Constant `SUN_RADIUS_KM`

```rust
pub const SUN_RADIUS_KM: f64 = 695700.0;
```

### Constant `MERCURY_EQUATORIAL_RADIUS_KM`

```rust
pub const MERCURY_EQUATORIAL_RADIUS_KM: f64 = 2440.5;
```

### Constant `MERCURY_POLAR_RADIUS_KM`

```rust
pub const MERCURY_POLAR_RADIUS_KM: f64 = 2438.3;
```

### Constant `VENUS_RADIUS_KM`

```rust
pub const VENUS_RADIUS_KM: f64 = 6051.8;
```

### Constant `EARTH_EQUATORIAL_RADIUS_KM`

```rust
pub const EARTH_EQUATORIAL_RADIUS_KM: f64 = 6378.1366;
```

### Constant `EARTH_FLATTENING`

```rust
pub const EARTH_FLATTENING: f64 = 0.996647180302104;
```

### Constant `EARTH_POLAR_RADIUS_KM`

```rust
pub const EARTH_POLAR_RADIUS_KM: f64 = 6356.751857971648;
```

### Constant `MOON_EQUATORIAL_RADIUS_KM`

```rust
pub const MOON_EQUATORIAL_RADIUS_KM: f64 = 1738.1;
```

### Constant `MOON_POLAR_RADIUS_KM`

```rust
pub const MOON_POLAR_RADIUS_KM: f64 = 1736.0;
```

### Constant `MARS_EQUATORIAL_RADIUS_KM`

```rust
pub const MARS_EQUATORIAL_RADIUS_KM: f64 = 3396.2;
```

### Constant `MARS_POLAR_RADIUS_KM`

```rust
pub const MARS_POLAR_RADIUS_KM: f64 = 3376.2;
```

### Constant `JUPITER_EQUATORIAL_RADIUS_KM`

```rust
pub const JUPITER_EQUATORIAL_RADIUS_KM: f64 = 71492.0;
```

### Constant `JUPITER_POLAR_RADIUS_KM`

```rust
pub const JUPITER_POLAR_RADIUS_KM: f64 = 66854.0;
```

### Constant `JUPITER_MEAN_RADIUS_KM`

```rust
pub const JUPITER_MEAN_RADIUS_KM: f64 = 69911.0;
```

### Constant `IO_RADIUS_KM`

```rust
pub const IO_RADIUS_KM: f64 = 1821.6;
```

### Constant `EUROPA_RADIUS_KM`

```rust
pub const EUROPA_RADIUS_KM: f64 = 1560.8;
```

### Constant `GANYMEDE_RADIUS_KM`

```rust
pub const GANYMEDE_RADIUS_KM: f64 = 2631.2;
```

### Constant `CALLISTO_RADIUS_KM`

```rust
pub const CALLISTO_RADIUS_KM: f64 = 2410.3;
```

### Constant `SATURN_EQUATORIAL_RADIUS_KM`

```rust
pub const SATURN_EQUATORIAL_RADIUS_KM: f64 = 60268.0;
```

### Constant `SATURN_POLAR_RADIUS_KM`

```rust
pub const SATURN_POLAR_RADIUS_KM: f64 = 54364.0;
```

### Constant `URANUS_EQUATORIAL_RADIUS_KM`

```rust
pub const URANUS_EQUATORIAL_RADIUS_KM: f64 = 25559.0;
```

### Constant `URANUS_POLAR_RADIUS_KM`

```rust
pub const URANUS_POLAR_RADIUS_KM: f64 = 24973.0;
```

### Constant `NEPTUNE_EQUATORIAL_RADIUS_KM`

```rust
pub const NEPTUNE_EQUATORIAL_RADIUS_KM: f64 = 24764.0;
```

### Constant `NEPTUNE_POLAR_RADIUS_KM`

```rust
pub const NEPTUNE_POLAR_RADIUS_KM: f64 = 24341.0;
```

### Constant `PLUTO_RADIUS_KM`

```rust
pub const PLUTO_RADIUS_KM: f64 = 1188.3;
```

### Constant `TIME_TEXT_BYTES`

```rust
pub const TIME_TEXT_BYTES: u32 = 28;
```

### Constant `astro_status_t_ASTRO_SUCCESS`

< The operation was successful.

```rust
pub const astro_status_t_ASTRO_SUCCESS: astro_status_t = 0;
```

### Constant `astro_status_t_ASTRO_NOT_INITIALIZED`

< A placeholder that can be used for data that is not yet initialized.

```rust
pub const astro_status_t_ASTRO_NOT_INITIALIZED: astro_status_t = 1;
```

### Constant `astro_status_t_ASTRO_INVALID_BODY`

< The celestial body was not valid. Different sets of bodies are supported depending on the function.

```rust
pub const astro_status_t_ASTRO_INVALID_BODY: astro_status_t = 2;
```

### Constant `astro_status_t_ASTRO_NO_CONVERGE`

< A numeric solver failed to converge. This should not happen unless there is a bug in Astronomy Engine.

```rust
pub const astro_status_t_ASTRO_NO_CONVERGE: astro_status_t = 3;
```

### Constant `astro_status_t_ASTRO_BAD_TIME`

< The provided date/time is outside the range allowed by this function.

```rust
pub const astro_status_t_ASTRO_BAD_TIME: astro_status_t = 4;
```

### Constant `astro_status_t_ASTRO_BAD_VECTOR`

< Vector magnitude is too small to be normalized into a unit vector.

```rust
pub const astro_status_t_ASTRO_BAD_VECTOR: astro_status_t = 5;
```

### Constant `astro_status_t_ASTRO_SEARCH_FAILURE`

< Search was not able to find an ascending root crossing of the function in the specified time interval.

```rust
pub const astro_status_t_ASTRO_SEARCH_FAILURE: astro_status_t = 6;
```

### Constant `astro_status_t_ASTRO_EARTH_NOT_ALLOWED`

< The Earth cannot be treated as a celestial body seen from an observer on the Earth itself.

```rust
pub const astro_status_t_ASTRO_EARTH_NOT_ALLOWED: astro_status_t = 7;
```

### Constant `astro_status_t_ASTRO_NO_MOON_QUARTER`

< No lunar quarter occurs inside the specified time range.

```rust
pub const astro_status_t_ASTRO_NO_MOON_QUARTER: astro_status_t = 8;
```

### Constant `astro_status_t_ASTRO_WRONG_MOON_QUARTER`

< Internal error: Astronomy_NextMoonQuarter found the wrong moon quarter.

```rust
pub const astro_status_t_ASTRO_WRONG_MOON_QUARTER: astro_status_t = 9;
```

### Constant `astro_status_t_ASTRO_INTERNAL_ERROR`

< A self-check failed inside the code somewhere, indicating a bug needs to be fixed.

```rust
pub const astro_status_t_ASTRO_INTERNAL_ERROR: astro_status_t = 10;
```

### Constant `astro_status_t_ASTRO_INVALID_PARAMETER`

< A parameter value passed to a function was not valid.

```rust
pub const astro_status_t_ASTRO_INVALID_PARAMETER: astro_status_t = 11;
```

### Constant `astro_status_t_ASTRO_FAIL_APSIS`

< Special-case logic for finding Neptune/Pluto apsis failed.

```rust
pub const astro_status_t_ASTRO_FAIL_APSIS: astro_status_t = 12;
```

### Constant `astro_status_t_ASTRO_BUFFER_TOO_SMALL`

< A provided buffer's size is too small to receive the requested data.

```rust
pub const astro_status_t_ASTRO_BUFFER_TOO_SMALL: astro_status_t = 13;
```

### Constant `astro_status_t_ASTRO_OUT_OF_MEMORY`

< An attempt to allocate memory failed.

```rust
pub const astro_status_t_ASTRO_OUT_OF_MEMORY: astro_status_t = 14;
```

### Constant `astro_status_t_ASTRO_INCONSISTENT_TIMES`

< The provided initial state vectors did not have matching times.

```rust
pub const astro_status_t_ASTRO_INCONSISTENT_TIMES: astro_status_t = 15;
```

### Constant `astro_body_t_BODY_INVALID`

< An invalid or undefined celestial body.

```rust
pub const astro_body_t_BODY_INVALID: astro_body_t = -1;
```

### Constant `astro_body_t_BODY_MERCURY`

< Mercury

```rust
pub const astro_body_t_BODY_MERCURY: astro_body_t = 0;
```

### Constant `astro_body_t_BODY_VENUS`

< Venus

```rust
pub const astro_body_t_BODY_VENUS: astro_body_t = 1;
```

### Constant `astro_body_t_BODY_EARTH`

< Earth

```rust
pub const astro_body_t_BODY_EARTH: astro_body_t = 2;
```

### Constant `astro_body_t_BODY_MARS`

< Mars

```rust
pub const astro_body_t_BODY_MARS: astro_body_t = 3;
```

### Constant `astro_body_t_BODY_JUPITER`

< Jupiter

```rust
pub const astro_body_t_BODY_JUPITER: astro_body_t = 4;
```

### Constant `astro_body_t_BODY_SATURN`

< Saturn

```rust
pub const astro_body_t_BODY_SATURN: astro_body_t = 5;
```

### Constant `astro_body_t_BODY_URANUS`

< Uranus

```rust
pub const astro_body_t_BODY_URANUS: astro_body_t = 6;
```

### Constant `astro_body_t_BODY_NEPTUNE`

< Neptune

```rust
pub const astro_body_t_BODY_NEPTUNE: astro_body_t = 7;
```

### Constant `astro_body_t_BODY_PLUTO`

< Pluto

```rust
pub const astro_body_t_BODY_PLUTO: astro_body_t = 8;
```

### Constant `astro_body_t_BODY_SUN`

< Sun

```rust
pub const astro_body_t_BODY_SUN: astro_body_t = 9;
```

### Constant `astro_body_t_BODY_MOON`

< Moon

```rust
pub const astro_body_t_BODY_MOON: astro_body_t = 10;
```

### Constant `astro_body_t_BODY_EMB`

< Earth/Moon Barycenter

```rust
pub const astro_body_t_BODY_EMB: astro_body_t = 11;
```

### Constant `astro_body_t_BODY_SSB`

< Solar System Barycenter

```rust
pub const astro_body_t_BODY_SSB: astro_body_t = 12;
```

### Constant `astro_body_t_BODY_STAR1`

< user-defined star #1

```rust
pub const astro_body_t_BODY_STAR1: astro_body_t = 101;
```

### Constant `astro_body_t_BODY_STAR2`

< user-defined star #2

```rust
pub const astro_body_t_BODY_STAR2: astro_body_t = 102;
```

### Constant `astro_body_t_BODY_STAR3`

< user-defined star #3

```rust
pub const astro_body_t_BODY_STAR3: astro_body_t = 103;
```

### Constant `astro_body_t_BODY_STAR4`

< user-defined star #4

```rust
pub const astro_body_t_BODY_STAR4: astro_body_t = 104;
```

### Constant `astro_body_t_BODY_STAR5`

< user-defined star #5

```rust
pub const astro_body_t_BODY_STAR5: astro_body_t = 105;
```

### Constant `astro_body_t_BODY_STAR6`

< user-defined star #6

```rust
pub const astro_body_t_BODY_STAR6: astro_body_t = 106;
```

### Constant `astro_body_t_BODY_STAR7`

< user-defined star #7

```rust
pub const astro_body_t_BODY_STAR7: astro_body_t = 107;
```

### Constant `astro_body_t_BODY_STAR8`

< user-defined star #8

```rust
pub const astro_body_t_BODY_STAR8: astro_body_t = 108;
```

### Constant `astro_refraction_t_REFRACTION_NONE`

< No atmospheric refraction correction (airless).

```rust
pub const astro_refraction_t_REFRACTION_NONE: astro_refraction_t = 0;
```

### Constant `astro_refraction_t_REFRACTION_NORMAL`

< Recommended correction for standard atmospheric refraction.

```rust
pub const astro_refraction_t_REFRACTION_NORMAL: astro_refraction_t = 1;
```

### Constant `astro_refraction_t_REFRACTION_JPLHOR`

< Used only for compatibility testing with JPL Horizons online tool.

```rust
pub const astro_refraction_t_REFRACTION_JPLHOR: astro_refraction_t = 2;
```

### Constant `astro_visibility_t_VISIBLE_MORNING`

< The body is best visible in the morning, before sunrise.

```rust
pub const astro_visibility_t_VISIBLE_MORNING: astro_visibility_t = 0;
```

### Constant `astro_visibility_t_VISIBLE_EVENING`

< The body is best visible in the evening, after sunset.

```rust
pub const astro_visibility_t_VISIBLE_EVENING: astro_visibility_t = 1;
```

### Constant `astro_apsis_kind_t_APSIS_PERICENTER`

< The body is at its closest approach to the object it orbits.

```rust
pub const astro_apsis_kind_t_APSIS_PERICENTER: astro_apsis_kind_t = 0;
```

### Constant `astro_apsis_kind_t_APSIS_APOCENTER`

< The body is at its farthest distance from the object it orbits.

```rust
pub const astro_apsis_kind_t_APSIS_APOCENTER: astro_apsis_kind_t = 1;
```

### Constant `astro_apsis_kind_t_APSIS_INVALID`

< Undefined or invalid apsis.

```rust
pub const astro_apsis_kind_t_APSIS_INVALID: astro_apsis_kind_t = 2;
```

### Constant `astro_eclipse_kind_t_ECLIPSE_NONE`

< No eclipse found.

```rust
pub const astro_eclipse_kind_t_ECLIPSE_NONE: astro_eclipse_kind_t = 0;
```

### Constant `astro_eclipse_kind_t_ECLIPSE_PENUMBRAL`

< A penumbral lunar eclipse. (Never used for a solar eclipse.)

```rust
pub const astro_eclipse_kind_t_ECLIPSE_PENUMBRAL: astro_eclipse_kind_t = 1;
```

### Constant `astro_eclipse_kind_t_ECLIPSE_PARTIAL`

< A partial lunar/solar eclipse.

```rust
pub const astro_eclipse_kind_t_ECLIPSE_PARTIAL: astro_eclipse_kind_t = 2;
```

### Constant `astro_eclipse_kind_t_ECLIPSE_ANNULAR`

< An annular solar eclipse. (Never used for a lunar eclipse.)

```rust
pub const astro_eclipse_kind_t_ECLIPSE_ANNULAR: astro_eclipse_kind_t = 3;
```

### Constant `astro_eclipse_kind_t_ECLIPSE_TOTAL`

< A total lunar/solar eclipse.

```rust
pub const astro_eclipse_kind_t_ECLIPSE_TOTAL: astro_eclipse_kind_t = 4;
```

### Constant `astro_aberration_t_ABERRATION`

< Request correction for aberration.

```rust
pub const astro_aberration_t_ABERRATION: astro_aberration_t = 0;
```

### Constant `astro_aberration_t_NO_ABERRATION`

< Do not correct for aberration.

```rust
pub const astro_aberration_t_NO_ABERRATION: astro_aberration_t = 1;
```

### Constant `astro_equator_date_t_EQUATOR_J2000`

< Represent equatorial coordinates in the J2000 epoch.

```rust
pub const astro_equator_date_t_EQUATOR_J2000: astro_equator_date_t = 0;
```

### Constant `astro_equator_date_t_EQUATOR_OF_DATE`

< Represent equatorial coordinates using the Earth's equator at the given date and time.

```rust
pub const astro_equator_date_t_EQUATOR_OF_DATE: astro_equator_date_t = 1;
```

### Constant `astro_direction_t_DIRECTION_RISE`

< Search for the time a body begins to rise above the horizon.

```rust
pub const astro_direction_t_DIRECTION_RISE: astro_direction_t = 1;
```

### Constant `astro_direction_t_DIRECTION_SET`

< Search for the time a body finishes sinking below the horizon.

```rust
pub const astro_direction_t_DIRECTION_SET: astro_direction_t = -1;
```

### Constant `astro_time_format_t_TIME_FORMAT_DAY`

< Truncate to UTC calendar date only, e.g. `2020-12-31`. Buffer size must be at least 11 characters.

```rust
pub const astro_time_format_t_TIME_FORMAT_DAY: astro_time_format_t = 0;
```

### Constant `astro_time_format_t_TIME_FORMAT_MINUTE`

< Round to nearest UTC minute, e.g. `2020-12-31T15:47Z`. Buffer size must be at least 18 characters.

```rust
pub const astro_time_format_t_TIME_FORMAT_MINUTE: astro_time_format_t = 1;
```

### Constant `astro_time_format_t_TIME_FORMAT_SECOND`

< Round to nearest UTC second, e.g. `2020-12-31T15:47:32Z`. Buffer size must be at least 21 characters.

```rust
pub const astro_time_format_t_TIME_FORMAT_SECOND: astro_time_format_t = 2;
```

### Constant `astro_time_format_t_TIME_FORMAT_MILLI`

< Round to nearest UTC millisecond, e.g. `2020-12-31T15:47:32.397Z`. Buffer size must be at least 25 characters.

```rust
pub const astro_time_format_t_TIME_FORMAT_MILLI: astro_time_format_t = 3;
```

### Constant `astro_node_kind_t_INVALID_NODE`

< Placeholder value for a missing or invalid node.

```rust
pub const astro_node_kind_t_INVALID_NODE: astro_node_kind_t = 0;
```

### Constant `astro_node_kind_t_ASCENDING_NODE`

< The body passes through the ecliptic plane from south to north.

```rust
pub const astro_node_kind_t_ASCENDING_NODE: astro_node_kind_t = 1;
```

### Constant `astro_node_kind_t_DESCENDING_NODE`

< The body passes through the ecliptic plane from north to south.

```rust
pub const astro_node_kind_t_DESCENDING_NODE: astro_node_kind_t = -1;
```

