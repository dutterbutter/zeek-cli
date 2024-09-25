use crate::rpc::proofs::StorageProof;
use anyhow::Result;
use colored::*;

pub fn visualize_merkle_path(proof: &StorageProof) -> Result<()> {
    println!("{}", "Merkle Tree Visualization:".bold());

    // Build the path from leaf to root
    let mut index = proof.index;
    let mut path = Vec::new();

    for sibling_hash in &proof.proof {
        let is_left = (index & 1) == 1;
        path.push((is_left, sibling_hash.clone()));
        index >>= 1;
    }

    // Reverse the path to start from the root
    path.reverse();

    // Helper function to print the tree
    fn print_tree(
        path: &[(bool, String)],
        prefix: String,
        is_left_child: bool,
        proof_value: &String,
    ) {
        if let Some((is_left, hash)) = path.first() {
            let connector = if is_left_child { "├──" } else { "└──" };
            let new_prefix = format!(
                "{}{}",
                prefix,
                if is_left_child { "│   " } else { "    " }
            );
            let label = if *is_left { "(Left Child)" } else { "(Right Child)" };

            // Apply colors
            let colored_hash = if *is_left {
                hash.green()
            } else {
                hash.cyan()
            };
            let colored_label = if *is_left {
                label.green()
            } else {
                label.cyan()
            };

            println!(
                "{}{} {} {}",
                prefix,
                connector,
                colored_hash.bold(),
                colored_label
            );
            print_tree(&path[1..], new_prefix, *is_left, proof_value);
        } else {
            let connector = if is_left_child { "├──" } else { "└──" };
            println!(
                "{}{} {} {}",
                prefix,
                connector,
                "Leaf Node (Value):".yellow().bold(),
                proof_value.yellow()
            );
        }
    }

    // Start printing from the root
    println!("{}", "Root Hash".bold().blue());
    if !path.is_empty() {
        print_tree(&path, "".to_string(), true, &proof.value);
    } else {
        println!(
            "{} {}",
            "└── Leaf Node (Value):".yellow().bold(),
            proof.value.yellow()
        );
    }

    Ok(())
}
