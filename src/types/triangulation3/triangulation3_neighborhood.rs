use types::N3Index;
use types::T4Index;
use types::Tetrahedron;

use algorithms3::sort_3::sort_3;

pub struct Triangulation3Neighborhood {
    triangle_neighborhood: Vec<Vec<(N3Index, Vec<(N3Index, Option<T4Index>, Option<T4Index>)>)>>,
}

impl Triangulation3Neighborhood {
    pub fn new() -> Triangulation3Neighborhood {
        Triangulation3Neighborhood { triangle_neighborhood: Vec::new() }
    }

    pub fn register_tetrahedron(&mut self, tetra: &Tetrahedron, tetra_index: T4Index) {
        for edge_indices in tetra.faces_as_indices_tuples().iter() {
            self.register_connection(edge_indices.0, edge_indices.1, edge_indices.2, tetra_index);
        }
    }

    pub fn get_neighbor(&self, p1: N3Index, p2: N3Index, p3: N3Index, tetra_index: T4Index) -> Option<T4Index> {
        let (smaller, medium, larger) = sort_3(p1, p2, p3);

        let v = &self.triangle_neighborhood[smaller.0];

        for i in 0..v.len() {
            let medium_v_item = &v[i];

            if medium_v_item.0 == medium {
                for i in 0..medium_v_item.1.len() {
                    let e = &medium_v_item.1[i];

                    if e.0 == larger {
                        if e.1 == Some(tetra_index) {
                            return e.2;
                        } else {
                            return e.1;
                        }
                    }
                }
            }
        }

        None
    }

    pub fn teach_triangles_of_neighborhood(elements: &mut [Tetrahedron]) {
        let mut neighborhood = Triangulation3Neighborhood::new();
        for i in 0..elements.len() {
            let e = &elements[i];
            neighborhood.register_tetrahedron(e, T4Index(i));
        }

        for n_smaller_index in 0..neighborhood.triangle_neighborhood.len() {
            for &(n_medium_index, ref innest_vec) in neighborhood.triangle_neighborhood[n_smaller_index].iter() {
                for &(n_largest_index, opt_t1, opt_t2) in innest_vec.iter() {
                    if let (Some(t1), Some(t2)) = (opt_t1, opt_t2) {
                        {
                            let el1: &mut Tetrahedron = &mut elements[t1.0];
                            let neighbor_index = el1.get_neighbor_index(N3Index(n_smaller_index), n_medium_index, n_largest_index);

                            el1.set_neighbor(neighbor_index, Some(t2));
                        }
                        {
                            let el2: &mut Tetrahedron = &mut elements[t2.0];
                            let neighbor_index = el2.get_neighbor_index(N3Index(n_smaller_index), n_medium_index, n_largest_index);

                            el2.set_neighbor(neighbor_index, Some(t1));
                        }
                    }
                }
            }
        }
    }

    fn register_connection(&mut self, p1: N3Index, p2: N3Index, p3: N3Index, tetra_index: T4Index) {
        let (smaller, medium, larger) =  sort_3(p1, p2, p3);

        if self.triangle_neighborhood.len() < larger.0 {
            self.triangle_neighborhood.resize(larger.0, Vec::new());
        }

        let v = &mut self.triangle_neighborhood[smaller.0];

        for i in 0..v.len() {
            let medium_v_item = &mut v[i];

            if medium_v_item.0 == medium {
                for i in 0..medium_v_item.1.len() {
                    let e = &mut medium_v_item.1[i];

                    if e.0 == larger {
                        assert!(e.1.is_some());
                        assert!(e.2.is_none());
                        e.2 = Some(tetra_index);

                        return;
                    }
                }

                medium_v_item.1.push((larger, Some(tetra_index), None));
                return;
            }
        }

        v.push((medium, vec!((larger, Some(tetra_index), None))));
    }
}

#[cfg(test)]
mod tests {
    use types::Point3;
    use types::Tetrahedron;
    use types::N3Index;
    use types::T4Index;
    use super::*;

    use types::triangulation3_test_utils::get_example_initial_point_set;
    use types::triangulation3::triangulation3_initiation::create_initial_tetra_set;

    #[test]
    fn testing_neighborhood() {
        let points = vec![Point3::new(0., 0., 0.), Point3::new(100., 0., 0.), Point3::new(0., 100., 0.), Point3::new(0., 0., 100.), Point3::new(100., 100., 0.)];

        let t0 = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let t1 = Tetrahedron::new(&points, N3Index(1), N3Index(2), N3Index(3), N3Index(4));

        let mut neighborhood = Triangulation3Neighborhood::new();

        neighborhood.register_tetrahedron(&t0, T4Index(0));
        neighborhood.register_tetrahedron(&t1, T4Index(1));

        assert_eq!(Option::None, neighborhood.get_neighbor(N3Index(0), N3Index(1), N3Index(2), T4Index(0)));
        assert_eq!(Some(T4Index(1)), neighborhood.get_neighbor(N3Index(1), N3Index(2), N3Index(3), T4Index(0)));
        assert_eq!(Option::None, neighborhood.get_neighbor(N3Index(2), N3Index(3), N3Index(0), T4Index(0)));
        assert_eq!(Option::None, neighborhood.get_neighbor(N3Index(3), N3Index(0), N3Index(1), T4Index(0)));

        assert_eq!(Some(T4Index(1)), neighborhood.get_neighbor(N3Index(3), N3Index(2), N3Index(1), T4Index(0)));


        assert_eq!(Some(T4Index(0)), neighborhood.get_neighbor(N3Index(1), N3Index(2), N3Index(3), T4Index(1)));
        assert_eq!(None, neighborhood.get_neighbor(N3Index(2), N3Index(3), N3Index(4), T4Index(1)));
        assert_eq!(None, neighborhood.get_neighbor(N3Index(3), N3Index(4), N3Index(1), T4Index(1)));
        assert_eq!(None, neighborhood.get_neighbor(N3Index(4), N3Index(1), N3Index(2), T4Index(1)));

        let mut tr = vec![t0, t1];

        //todo more tests.

        //TriangulationNeighborhood::teach_triangles_of_neighborhood(&mut tr);

        /*assert_eq!(Some(T4Index(1)), tr[0].get_neighbor_from_index(1));
        assert_eq!(None, tr[0].get_neighbor_from_index(0));
        assert_eq!(None, tr[0].get_neighbor_from_index(2));

        assert_eq!(None, tr[1].get_neighbor_from_index(1));
        assert_eq!(Some(T4Index(0)), tr[1].get_neighbor_from_index(0));
        assert_eq!(None, tr[1].get_neighbor_from_index(2));*/
    }

    #[test]
    fn testing_neighborhood_with_initiation() {
        let nodes = get_example_initial_point_set();
        let mut eles: Vec<Tetrahedron> = create_initial_tetra_set(&nodes);

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut eles);

        /* cheat sheet
        assert_eq!(&[N3Index(0),N3Index(3),N3Index(4),N3Index(1)],tetras[0].nodes());
        assert_eq!(&[N3Index(1),N3Index(2),N3Index(3),N3Index(6)],tetras[1].nodes());
        assert_eq!(&[N3Index(1),N3Index(4),N3Index(5),N3Index(6)],tetras[2].nodes());
        assert_eq!(&[N3Index(3),N3Index(4),N3Index(6),N3Index(7)],tetras[3].nodes());
        assert_eq!(&[N3Index(1),N3Index(3),N3Index(4),N3Index(6)],tetras[4].nodes());
        */

        assert_eq!(None,eles[0].get_neighbor_for_indices(N3Index(0),N3Index(3),N3Index(4)));
        assert_eq!(Some(T4Index(4)),eles[0].get_neighbor_for_indices(N3Index(3),N3Index(4),N3Index(1)));
        assert_eq!(None,eles[0].get_neighbor_for_indices(N3Index(4),N3Index(1),N3Index(0)));
        assert_eq!(None,eles[0].get_neighbor_for_indices(N3Index(1),N3Index(0),N3Index(3)));

        assert_eq!(None,eles[1].get_neighbor_for_indices(N3Index(1),N3Index(2),N3Index(3)));
        assert_eq!(None,eles[1].get_neighbor_for_indices(N3Index(2),N3Index(3),N3Index(6)));
        assert_eq!(Some(T4Index(4)),eles[1].get_neighbor_for_indices(N3Index(3),N3Index(6),N3Index(1)));
        assert_eq!(None,eles[1].get_neighbor_for_indices(N3Index(6),N3Index(1),N3Index(2)));

        assert_eq!(None,eles[2].get_neighbor_for_indices(N3Index(1),N3Index(4),N3Index(5)));
        assert_eq!(None,eles[2].get_neighbor_for_indices(N3Index(4),N3Index(5),N3Index(6)));
        assert_eq!(None,eles[2].get_neighbor_for_indices(N3Index(5),N3Index(6),N3Index(1)));
        assert_eq!(Some(T4Index(4)),eles[2].get_neighbor_for_indices(N3Index(6),N3Index(1),N3Index(4)));

        assert_eq!(Some(T4Index(4)),eles[3].get_neighbor_for_indices(N3Index(3),N3Index(4),N3Index(6)));
        assert_eq!(None,eles[3].get_neighbor_for_indices(N3Index(4),N3Index(6),N3Index(7)));
        assert_eq!(None,eles[3].get_neighbor_for_indices(N3Index(6),N3Index(7),N3Index(3)));
        assert_eq!(None,eles[3].get_neighbor_for_indices(N3Index(7),N3Index(3),N3Index(4)));

        assert_eq!(Some(T4Index(0)),eles[4].get_neighbor_for_indices(N3Index(1),N3Index(3),N3Index(4)));
        assert_eq!(Some(T4Index(3)),eles[4].get_neighbor_for_indices(N3Index(3),N3Index(4),N3Index(6)));
        assert_eq!(Some(T4Index(2)),eles[4].get_neighbor_for_indices(N3Index(4),N3Index(6),N3Index(1)));
        assert_eq!(Some(T4Index(1)),eles[4].get_neighbor_for_indices(N3Index(6),N3Index(1),N3Index(3)));
    }
}