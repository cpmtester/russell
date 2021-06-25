#[derive(Debug)]
pub struct Histogram<'a, T>
where
    T: Into<f64> + Copy,
{
    stations: &'a [T],
    counts: Vec<usize>,
}

impl<'a, T> Histogram<'a, T>
where
    T: Into<f64> + Copy,
{
    pub fn new(stations: &'a [T]) -> Self {
        Histogram {
            stations,
            counts: Vec::new(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_new_works() {
        let stations = [0, 1, 2, 3, 4, 5];
        let hist = Histogram::new(&stations);
        assert_eq!(hist.stations, &[0, 1, 2, 3, 4, 5])
    }
}
