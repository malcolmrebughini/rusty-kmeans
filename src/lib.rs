// select random points form data to use as centers
// group by proximity to centers
// calculate new centers based on group
// calculate distances again and reassign
// continue until the grouping doesn't change anymore

// Config keys based on sklearn interface
struct Config {
    n_clusters: u32,
}

impl Config {
    pub fn default() -> Config {
        let config = Config { n_clusters: 8 };

        return config;
    }

    pub fn n_clusters(mut self, n_clusters: u32) -> Self {
        self.n_clusters = n_clusters;
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

    pub fn fit(self, data: Vec<(f64, f64)>) -> (Vec<u32>, Vec<(f64, f64)>) {
        return (vec![], vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit() {
        let config = Config::default();
        let kmeans = KMeans::new(config);
        let data = vec![(1., 2.), (1., 4.), (1., 0.), (4., 2.), (4., 4.), (4., 0.)];
        let (labels, centers) = kmeans.fit(data);

        assert_eq!(labels, vec![0, 0, 0, 1, 1, 1]);
        assert_eq!(centers, vec![(1., 2.), (4., 2.)]);
    }
}
