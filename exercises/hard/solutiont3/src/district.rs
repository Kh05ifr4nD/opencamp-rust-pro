use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Debug)]
struct MapCity2Edge(HashMap<String, Vec<String>>);
impl<'d> Deserialize<'d> for MapCity2Edge {
  fn deserialize<D: Deserializer<'d>>(deserer: D) -> Result<Self, D::Error> {
    struct Visitor;

    impl<'d> serde::de::Visitor<'d> for Visitor {
      type Value = MapCity2Edge;

      fn expecting(&self, fmtter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmtter.write_str("a map that maps city names to their list of adjacent cities")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::MapAccess<'d>,
      {
        let mut data = HashMap::with_capacity(24);

        while let Some((k, v)) = map.next_entry::<String, Vec<String>>()? {
          data
            .entry(k)
            .and_modify(|e: &mut Vec<String>| e.extend(v.iter().cloned()))
            .or_insert_with(|| v);
        }

        Ok(MapCity2Edge(data))
      }
    }

    deserer.deserialize_map(Visitor)
  }
}

#[derive(Deserialize, Debug)]
struct MapBatch {
  #[serde(flatten)]
  map: HashMap<String, MapCity2Edge>,
}

pub fn count_prov() -> String {
  #[inline]
  const fn find(prnt: &mut [usize], x: usize) -> usize {
    let mut root = x;
    while prnt[root] != root {
      prnt[root] = prnt[prnt[root]];
      root = prnt[root];
    }
    root
  }

  #[inline]
  const fn union(prnt: &mut [usize], rk: &mut [usize], x: usize, y: usize) {
    let root_x = find(prnt, x);
    let root_y = find(prnt, y);
    if root_x != root_y {
      if rk[root_x] > rk[root_y] {
        prnt[root_y] = root_x;
      } else {
        prnt[root_x] = root_y;
        if rk[root_x] == rk[root_y] {
          rk[root_y] += 1;
        }
      }
    }
  }

  let map_batch: MapBatch =
    serde_json::from_str(&std::fs::read_to_string("district.json").unwrap()).unwrap();
  let mut map_batch = map_batch.map.iter().collect::<Vec<_>>();
  map_batch.sort_by_key(|&(k, _)| k.parse::<usize>().unwrap());

  let mut rslt = Vec::with_capacity(5);

  for (_, batch) in map_batch {
    let mut map_city2idx = HashMap::new();
    let mut i = 0;

    for (city, ls_city_adj) in &batch.0 {
      if !map_city2idx.contains_key(city) {
        map_city2idx.insert(city, i);
        i += 1;
      }
      for city_adj in ls_city_adj {
        if !map_city2idx.contains_key(city_adj) {
          map_city2idx.insert(city_adj, i);
          i += 1;
        }
      }
    }

    let n = map_city2idx.len();
    let mut prnt = (0..n).collect::<Vec<_>>();
    let mut rk = vec![0; n];

    for (city, ls_city_adj) in &batch.0 {
      let x = *map_city2idx.get(city).unwrap();
      for c in ls_city_adj {
        let y = *map_city2idx.get(c).unwrap();
        union(&mut prnt, &mut rk, x, y);
      }
    }

    let cnt = (0..n).filter(|&i| prnt[i] == i).count();

    rslt.push(cnt.to_string());
  }

  rslt.join(",")
}
