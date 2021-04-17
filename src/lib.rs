use rand;
use std::collections::HashMap;

type Point = (f64, f64);
type Centers = Vec<Point>;
type Labels = Vec<usize>;

// Can be moved to its own module
fn calculate_euclidian_distance(p: Point, q: Point) -> f64 {
    let (x1, y1) = p;
    let (x2, y2) = q;

    let x = x1 - x2;
    let y = y1 - y2;

    return (x * x + y * y).sqrt();
}

fn init_random(data: &Vec<Point>, count: usize) -> Centers {
    let mut rng = rand::thread_rng();
    let rnd_indexes = rand::seq::index::sample(&mut rng, data.len(), count);
    return rnd_indexes.iter().map(|i| data[i]).collect();
}

enum Init {
    // Could add more, like PlusPlus
    Random,
    Fixed(Centers), // Fixed centers for the initial pass
}
// Config keys based on sklearn interface
struct Config {
    n_clusters: usize,
    init: Init,
}

impl Config {
    pub fn default() -> Config {
        let config = Config {
            n_clusters: 8,
            init: Init::Random,
        };

        return config;
    }

    pub fn n_clusters(mut self, n_clusters: usize) -> Self {
        self.n_clusters = n_clusters;
        return self;
    }

    pub fn init(mut self, init: Init) -> Self {
        self.init = init;
        return self;
    }
}

struct KMeans {
    config: Config,
}

impl KMeans {
    pub fn new(config: Config) -> KMeans {
        return KMeans { config: config };
    }

    // Deviating a bit from the sklearn interface and returning the labels
    // and centers instead of saving them internally in the KMeans object.
    pub fn fit(self, data: Vec<Point>) -> (Labels, Centers) {
        // TODO: Check data length is more than 0
        // TODO: Check data length is less than the desired clusters
        let mut centers = match self.config.init {
            Init::Random => init_random(&data, self.config.n_clusters),
            Init::Fixed(centers) => centers,
        };

        let mut labels: Labels = vec![0; data.len()];
        let mut distances: Vec<f64> = vec![0.; data.len()];

        // Calculate distances and assign labels
        for (i, point) in data.iter().enumerate() {
            let mut closest_distance: Option<f64> = None;
            let mut closest_index: usize = 0;

            for (y, center) in centers.iter().enumerate() {
                let distance = calculate_euclidian_distance(*point, *center);

                if closest_distance == None || distance < closest_distance.unwrap() {
                    closest_distance = Some(distance);
                    closest_index = y
                }
            }

            labels[i] = closest_index;
            distances[i] = closest_distance.unwrap();
        }

        let mut new_centers = vec![(0., 0.); centers.len()];

        while new_centers != centers {
            centers = new_centers;
            new_centers = vec![(0., 0.); centers.len()];

            // calculate new centers based on previously assigned clusters
            // map to sum all points by cluster index and calculate new center points
            let mut points_accumulator: HashMap<usize, (usize, f64, f64)> = HashMap::new();

            for (i, (p1, p2)) in data.iter().enumerate() {
                let cluster_index = labels[i];

                // retrieve entry for the cluster accumulator or add default
                let acc = points_accumulator
                    .entry(cluster_index)
                    .or_insert((0, 0., 0.));

                // add values to entry
                *acc = (acc.0 + 1, acc.1 + p1, acc.2 + p2);
            }

            // calculate and assign new centers in same place in Vec
            for (i, (count, x, y)) in points_accumulator.iter() {
                new_centers[*i] = (*x / (*count) as f64, *y / (*count) as f64);
            }

            // Calculate distances again and change labels if required
            for (i, point) in data.iter().enumerate() {
                let mut closest_distance: Option<f64> = Some(distances[i]);
                let mut closest_index: usize = labels[i];

                for (y, center) in new_centers.iter().enumerate() {
                    let distance = calculate_euclidian_distance(*point, *center);

                    if closest_distance == None || distance < closest_distance.unwrap() {
                        closest_distance = Some(distance);
                        closest_index = y
                    }
                }

                labels[i] = closest_index;
                distances[i] = closest_distance.unwrap();
            }
        }

        return (labels, centers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_fixed_init() {
        let config = Config::default()
            .n_clusters(2)
            .init(Init::Fixed(vec![(1., 2.), (4., 2.)]));
        let kmeans = KMeans::new(config);
        let data = vec![(1., 2.), (1., 4.), (1., 0.), (4., 2.), (4., 4.), (4., 0.)];
        let (labels, centers) = kmeans.fit(data);

        assert_eq!(labels, vec![0, 0, 0, 1, 1, 1]);
        assert_eq!(centers, vec![(1., 2.), (4., 2.)]);
    }
}
