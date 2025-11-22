use std::{error::Error};

fn cross(parent_a: &Vec<usize>, parent_b: &Vec<usize>, mixup_low: usize, mixup_higher: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    assert!(mixup_higher >= mixup_low);
    assert!(parent_a.len() == parent_b.len());

    let mut child = parent_b.clone();

    let mut parent_a_gene_index: usize = 0;
    let mut child_gene_index: usize = 0;

    while parent_a_gene_index < parent_a.len() && child_gene_index < child.len() {
        cross_find_next_gene_index(parent_a, mixup_low, mixup_higher, &mut parent_a_gene_index);
        cross_find_next_gene_index(&child, mixup_low, mixup_higher, &mut child_gene_index);

        if parent_a_gene_index >= parent_a.len() || child_gene_index >= child.len() {
            break;
        }

        child[child_gene_index] = parent_a[parent_a_gene_index];
        parent_a_gene_index += 1;
        child_gene_index += 1;
    }

    Ok(child)
}

fn cross_find_next_gene_index(specie_gens: &Vec<usize>, mixup_low: usize, mixup_higher: usize, gene_index: &mut usize) {
    while *gene_index < specie_gens.len() && specie_gens[*gene_index] < mixup_low || specie_gens[*gene_index] > mixup_higher {
        *gene_index += 1;
    }
}

fn mutate(child: Vec<usize>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        let arr_a = vec![1,5,0,4,6,2,3];
        let arr_b = vec![5,3,6,1,4,2,0];
        let child = cross(&arr_a, &arr_b, 3, 5).unwrap();
        assert_eq!(child, [5,4,6,1,3,2,0])
    }
}