#[derive(Debug)]
pub struct Histogram<'a, T>
where
    T: Into<f64> + Copy + PartialOrd,
{
    stations: &'a [T],
    counts: Vec<usize>,
}

impl<'a, T> Histogram<'a, T>
where
    T: Into<f64> + Copy + PartialOrd,
{
    pub fn new(stations: &'a [T]) -> Self {
        Histogram {
            stations,
            counts: Vec::new(),
        }
    }

    pub fn find_station(&self, val: T) -> Option<usize> {
        let n = self.stations.len();
        if n < 2 {
            return None; // not enough stations
        }
        if val < self.stations[0] {
            return None; // out-of-bounds
        }
        if val >= self.stations[n - 1] {
            return None; // out-of-bounds
        }

        // perform binary search
        let mut upper = n;
        let mut lower = 0;
        let mut mid: usize;
        while upper - lower > 1 {
            mid = (upper + lower) / 2;
            if val >= self.stations[mid] {
                lower = mid
            } else {
                upper = mid
            }
        }
        Some(lower)
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
