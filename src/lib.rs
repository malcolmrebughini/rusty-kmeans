// select random points form data to use as centers
// group by proximity to centers
// calculate new centers based on group
// calculate distances again and reassign
// continue until the grouping doesn't change anymore

// Config struct

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

    pub fn fit(self, data: Vec<(f64, f64)>) {}
}

// KMeans struct

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit() {
        let config = Config::default();
        let kmeans = KMeans::new(config);
        kmeans.fit(vec![]);
    }
}
