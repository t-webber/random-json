//! Define traits to apply the data generator on all sorts of types.

use core::hash::{Hash, Hasher};
use core::mem::discriminant;
use std::collections::{HashMap, HashSet};

use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;
use rand::{Rng as _, RngCore, rng};
use random_data::{DataGenerator, DataType};
use serde_json::{Number, Value};

use crate::errors::{Error, Res};
///
/// Generate random data of the given type.
pub trait Generator<T>: Sized {
    /// Generate random data of the given type.
    fn generate<Rng: RngCore>(&self, data: &mut Data<Rng>) -> Res<T>;
}

/// Generate random data of the given type, but with a nullable type.
pub trait NullableGenerator<T>: Sized {
    /// Generate random data of the given type, but with a nullable type.
    ///
    /// This can sometimes returns None.
    fn generate_nullable<Rng: RngCore>(&self, data: &mut Data<Rng>) -> Res<Option<T>>;
}

/// Contains the list of data types and the random generator to apply
/// generators.
pub struct Data<Rng: RngCore> {
    /// Radnom data generator
    random_data_generator: DataGenerator<Rng>,
    /// Pseudo-random refs
    ///
    /// This represents data that is randomly generated once, then used in
    /// multiple place.
    refs: HashMap<String, OutputData>,
    /// Random generator
    rng: ThreadRng,
    /// Data types that were required to be unique.
    uniq_types: HashMap<String, HashSet<OutputData>>,
    /// User-defined data types
    user_defined: HashMap<String, Vec<String>>,
}

impl<Rng: RngCore> Data<Rng> {
    /// Generate non-nullable data of the provided data type.
    fn generate(&mut self, data_type: &str) -> Res<OutputData> {
        if let Some(parsed) = data_type.strip_suffix(']')
            && let Some(pos) = parsed.rfind('[')
        {
            return self.generate_ref(parsed, pos);
        }
        if let Some(parsed) = data_type.strip_suffix('*') {
            return self.generate_unique(parsed);
        }
        if data_type.contains("..") {
            return self.generate_range(data_type);
        }
        if data_type.contains('|') {
            return self.generate_enum(data_type);
        }

        let value = if let Some(values) = self.user_defined.get(data_type) {
            OutputData::String(
                values
                    .choose(&mut self.rng)
                    .ok_or(Error::FakerDefEmpty)?
                    .to_owned(),
            )
        } else if data_type == "Bool" {
            OutputData::Bool(self.rng.random_bool(0.5))
        } else if data_type == "Int" {
            OutputData::Int(self.rng.random_range(0..=u64::MAX))
        } else if data_type == "Float" {
            OutputData::Float(self.rng.random_range(0.0f64..=f64::MAX))
        } else {
            OutputData::String(
                DataType::try_from(data_type)
                    .map_err(|()| Error::InvalidDataType(data_type.to_owned()))?
                    .random(&mut self.random_data_generator),
            )
        };

        Ok(value)
    }

    /// Generate a user-defined data-type, defined with `|`
    fn generate_enum(&mut self, data_type: &str) -> Res<OutputData> {
        let values = data_type
            .split('|')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        values
            .choose(self.rng())
            .ok_or(Error::MissingValueBeforePipe)
            .map(|data| OutputData::String((*data).to_owned()))
    }

    /// Generate nullable data of the provided data type.
    fn generate_nullable(&mut self, data_type: &str) -> Res<Option<OutputData>> {
        let parsed_data_type = if let Some(parsed_data_type) = data_type.strip_suffix('?') {
            if self.random_null() {
                return Ok(None);
            }
            parsed_data_type
        } else {
            data_type
        };

        self.generate(parsed_data_type).map(Some)
    }

    /// Generate the data for a range of numbers instead of a data type
    #[expect(clippy::unwrap_used, reason = ".. in string")]
    fn generate_range(&mut self, data_type: &str) -> Res<OutputData> {
        let mut split = data_type.split("..");
        let min_str = split.next().unwrap();
        if let Ok(min) = min_str.parse() {
            let max = if let Some(max_str) = split.next() {
                max_str
                    .parse::<u64>()
                    .map_err(Error::invalid_bounds(|| min_str.to_owned()))?
            } else {
                u64::MAX
            };

            return Ok(OutputData::Int(self.rng().random_range(min..max)));
        }
        match min_str.parse() {
            Ok(min) => {
                let max = if let Some(max_str) = split.next() {
                    max_str
                        .parse::<f64>()
                        .map_err(Error::invalid_bounds(|| max_str.to_owned()))?
                } else {
                    f64::MAX
                };
                Ok(OutputData::Float(self.rng().random_range(min..max)))
            }
            Err(error) => Err(Error::invalid_bounds(|| min_str.to_owned())(error)),
        }
    }

    /// Generate random data with a given ref
    fn generate_ref(&mut self, data_type: &str, ref_position: usize) -> Res<OutputData> {
        let (type_name, ref_name) = data_type.split_at(ref_position);
        let key = ref_name.get(1..).unwrap_or_default();
        if let Some(value) = self.refs.get(key) {
            Ok(value.to_owned())
        } else {
            let value = self.generate(type_name)?;
            self.refs.insert(key.to_owned(), value.clone());
            Ok(value)
        }
    }

    /// Generate a data type that must be different at every generation.
    #[expect(clippy::unwrap_used, reason = "generate can't empty uniq_types")]
    fn generate_unique(&mut self, data_type: &str) -> Res<OutputData> {
        if self.uniq_types.contains_key(data_type) {
            for _ in 0..10_000 {
                let generated_data = self.generate(data_type)?;
                let banned = self.uniq_types.get_mut(data_type).unwrap();
                if !banned.contains(&generated_data) {
                    banned.insert(generated_data.clone());
                    return Ok(generated_data);
                }
            }
            let already_produced = self.uniq_types.get_mut(data_type).unwrap().len();
            Err(Error::UniqueFetchFailed { data_type: data_type.to_owned(), already_produced })
        } else {
            let generated_data = self.generate(data_type)?;
            self.uniq_types
                .insert(data_type.to_owned(), HashSet::from([generated_data.clone()]));
            Ok(generated_data)
        }
    }

    /// List all the data types, user defined and from `random-data`.
    pub fn list(&self) -> Vec<String> {
        let random_data_types = DataType::list_str();

        let mut list = Vec::with_capacity(
            self.user_defined
                .len()
                .saturating_add(random_data_types.len()),
        );

        self.user_defined.keys().for_each(|key| {
            list.push(key.to_owned());
        });

        for data_type in random_data_types {
            list.push((*data_type).to_owned());
        }

        list
    }

    /// Parse a user-defined data-type, with the format
    /// `Name:Value1|Value2|Value3`.
    fn parse_user_defined(user_input: &str) -> Res<(String, Vec<String>)> {
        let mut split = user_input.split(':');

        #[expect(clippy::unwrap_used, reason = "slipt always has first element")]
        let name = split.next().unwrap();

        let Some(values) = split.next() else {
            return Err(Error::FakerDefMissingColon);
        };

        if split.next().is_some() {
            return Err(Error::FakerDefTooManyColons);
        }

        Ok((name.to_owned(), values.split('|').map(str::to_owned).collect()))
    }

    /// Indicates whether to return null or the data for nullable types.
    pub fn random_null(&mut self) -> bool {
        self.rng.random_bool(0.3)
    }

    /// Borrows the random generator as mutable.
    pub const fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }

    /// List the possible values of a data-type
    pub fn values(&self, data_type: &str) -> Res<String> {
        if let Some(values) = self.user_defined.get(data_type) {
            Ok(values.join("\n"))
        } else {
            DataType::try_from(data_type)
                .map_err(|()| Error::InvalidDataType(data_type.to_owned()))?
                .values()
                .ok_or_else(|| Error::NonEnumerableDataType(data_type.to_owned()))
                .map(|list| list.join("\n"))
        }
    }
}

impl Data<ThreadRng> {
    /// Build the [`Data`] handler from user inputs
    pub fn new(input_data: Vec<String>) -> Res<Self> {
        let mut user_defined = HashMap::new();

        for data_type in input_data {
            let (name, values) = Self::parse_user_defined(&data_type)?;

            if user_defined.insert(name, values).is_some() {
                return Err(Error::DuplicateDataType(data_type));
            }
        }

        Ok(Self {
            random_data_generator: DataGenerator::default(),
            user_defined,
            rng: rng(),
            refs: HashMap::new(),
            uniq_types: HashMap::new(),
        })
    }
}

impl Generator<OutputData> for String {
    fn generate<Rng: RngCore>(&self, data: &mut Data<Rng>) -> Res<OutputData> {
        data.generate(self)
    }
}

impl NullableGenerator<OutputData> for String {
    fn generate_nullable<Rng: RngCore>(&self, data: &mut Data<Rng>) -> Res<Option<OutputData>> {
        data.generate_nullable(self)
    }
}

/// Output data of the data generator, modified to have the right type instead
/// of always string.
#[derive(Clone, PartialEq)]
pub enum OutputData {
    /// Output for "Bool"
    Bool(bool),
    /// Output for "Float" or float ranges
    Float(f64),
    /// Output for "Int" or integer ranges.
    Int(u64),
    /// Output for all the others
    String(String),
}

impl Eq for OutputData {}

impl Hash for OutputData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        match self {
            Self::Bool(bool) => bool.hash(state),
            Self::Float(float) => float.to_bits().hash(state),
            Self::Int(int) => int.hash(state),
            Self::String(string) => string.hash(state),
        }
    }
}

impl OutputData {
    /// Convert an [`OutputValue`] to the inner value in string format
    pub fn into_string(self) -> String {
        match self {
            Self::Bool(true) => "True".to_owned(),
            Self::Bool(false) => "False".to_owned(),
            Self::Float(number) => number.to_string(),
            Self::Int(number) => number.to_string(),
            Self::String(string) => string,
        }
    }
}

impl TryFrom<OutputData> for Value {
    type Error = Error;

    fn try_from(value: OutputData) -> Res<Self> {
        Ok(match value {
            OutputData::String(str) => Self::String(str),
            OutputData::Float(nb) =>
                Self::Number(Number::from_f64(nb).ok_or(Error::InfinityNotSupported)?),
            OutputData::Int(nb) => Self::Number(nb.into()),
            OutputData::Bool(bool) => Self::Bool(bool),
        })
    }
}
