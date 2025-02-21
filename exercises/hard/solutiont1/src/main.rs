// I AM NOT DONE

mod conjecture;

fn main() {
  let values = conjecture::goldbach_conjecture();
  println!("top 2 goldbach's conjecture on primes: {values}");
}
