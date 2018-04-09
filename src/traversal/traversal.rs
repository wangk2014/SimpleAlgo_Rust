use super::edge::*;

pub struct Traveral;


static mut edges: Vec<Edge> = Vec::new();
static mut heads: Vec<Head> = Vec::new();

impl Traveral {

    pub fn dfs_edge(&self ,point: i32){

        let  point:  usize = point as usize;
        {
            let  e: &mut Edge = edges.get_mut(point).unwrap();
            e.visited = true;
        }
        println!("dfs: {}", point);
        let  mut  i: i32;

        i = heads[point].from;
        while  i != -1 {
            {
                let  fe = edges.get_mut(point).unwrap();
//                let mut fe = edges[point];
                i = fe.next;

                if fe.visited == false {
                    Traveral::dfs_edge(self,  fe.to);
                }

            }

        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_permutations() {
        //Traveral::dfs_edge();
    }
}