
extern crate tbn;
extern crate rand;
extern crate hex;

mod users;
use users::{User, SurveyAuthority, RegistrationAuthority, VerificationKey};

use tbn::{Group, Fq, G1, Fq2, G2, Fr, pairing};
use tbn::arith::U256;

use hex::FromHex;

// Returns generators (g, g2) in (G1, G2)
// Because G1 and G2 are additive cyclic groups of prime order by construction of BN curves
// It is sufficient to randomly choose elements in G1 and G2 to get g and g2
fn get_generator_pair() -> (G1, G2) {
    
    // Crytpographiclaly secure thread-local rng
    let rng = &mut rand::thread_rng();

    // Generate random elements in G1 and G2
    let (mut g, mut g2):(G1, G2) = (G1::random(rng), G2::random(rng));
    // Ensure that g,g2 are both generators (i.e. non-zero in additive cyclic group of prime
    // order)
    while g.is_zero() {
        g = G1::random(rng);
    }
    while g2.is_zero() {
        g2 = G2::random(rng);
    }

    // Return generator pair
    (g, g2)
}

// Convert U256 into hex string encoding (excluding 0x)
fn to_hex_string(n:U256) -> String {

    let bytes = to_bytes(n);

    // Return hex encoding of byte vector
    return hex::encode(bytes);
}


// Iterate through bits of U256 and return byte vector in MSB order
fn to_bytes(n:U256) -> Vec<u8> {

    let mut iter = 0;
    let mut byte:u8 = 0;
    let mut bytes:Vec<u8> = vec![];
    for b in n.bits() {
        let bit = b as u8;
        // Finished whole byte -- save byte to vector and reset first
        if iter % 8 == 0 {
            bytes.push(byte);
            byte = 0;
        }
        byte += bit * u8::pow(2, 7 - (iter % 8));
        iter += 1;
    }
    bytes.push(byte);

    return bytes;
}

fn main() {
    
    /* ------------------------------------------------------------------------------
     *                          Barreto-Naehrig (BN) Curves                         
     * 
     * Pairing-friendly bilinear elliptic curve (see code for in-depth description)
     *
     * Sources:
     *  - Barreto-Naehrig Curves (Kasamatsu et al., 2014)
     *      https://tools.ietf.org/id/draft-kasamatsu-bncurves-01.html
     *  - A Family of Implementation-Friendly BN Elliptic Curves (Pereira et al., 2011)
     *      https://eprint.iacr.org/2010/429.pdf
     *
     * ------------------------------------------------------------------------------
     */

    const BN_BYTES:usize = 32;
    println!("256-bit Barreto-Naehrig curve (Fp256BN):");
    println!();
    println!("BN curves are bilinear pairings e : G1 × G2 -> Gt with:");
    let p:U256 = Fq::modulus();
    println!("\tp (prime modulus for elliptic curves) = 0x{}", to_hex_string(p));

    // Known q parameter (prime order of G1) for 256-bit BN curve (Kasamatsu et al., 2014)
    let q_hex = String::from("fffffffffffcf0cd46e5f25eee71a49e0cdc65fb1299921af62d536cd10b500d");
    let q_slice = <[u8; BN_BYTES]>::from_hex(q_hex.clone()).expect("Could not decode q");
    let q = U256::from_slice(&q_slice).expect("Could not convert q to U256"); 
    println!("\tq (prime order of G1, G2, and Gt) = 0x{}", q_hex);

    // TODO: Figure out what z does in G1 and G2
    
    println!("\tG1 = E/𝔽_q is a q-order additive cyclic subgroup of E(𝔽_p), where E : y^2 = x^3 + b\tmod p is an elliptic curve with:");
    println!("\t\t(x,y) ∈ E(𝔽_p) (base point):");
    let x:U256 = G1::one().x().into_u256();
    println!("\t\t\tx = 0x{}", to_hex_string(x));
    let y:U256 = G1::one().y().into_u256();
    println!("\t\t\ty = 0x{}", to_hex_string(y));
    let b:U256 = G1::b().into_u256();
    println!("\t\tb ∈ 𝔽_p (constant coefficient) = 0x{}", to_hex_string(b));
    println!();
    
    println!("\tG2 = E'/𝔽_q2 is an additive cyclic subgroup of E(𝔽_{{p^k}}), where E' : y^2 = x^3 + b/xi\tmod p  is a twisted elliptic curve with:");
    let mut k_slice:[u8;BN_BYTES] = [0;BN_BYTES];
    k_slice[BN_BYTES-1] = 12;
    let k:U256 = U256::from_slice(&k_slice).expect("Could not convert k to U256");
        println!("\t\tk (embedding degree of G2) = {}", to_hex_string(k));

    println!("\t\t(x,y) ∈ E(𝔽_{{p^k}}), (base point):");

    let base_pt:(Fq2, Fq2) = (G2::one().x(), G2::one().y());
    let x2_real:U256 = base_pt.0.real().into_u256();
    let x2_i:U256 = base_pt.0.imaginary().into_u256();
    println!("\t\t\tx = 0x{} + 0x{} i", to_hex_string(x2_real), to_hex_string(x2_i));
    let y2_real:U256 = base_pt.1.real().into_u256();
    let y2_i:U256 = base_pt.1.imaginary().into_u256();
    println!("\t\t\ty = 0x{} + 0x{} i", to_hex_string(y2_real), to_hex_string(y2_i));
    let b2_real:U256 = G2::b().real().into_u256();
    let b2_i:U256 = G2::b().imaginary().into_u256();    
    println!("\t\tb' ∈ 𝔽_q2 (constant coefficient) = 0x{} + 0x{} i", to_hex_string(b2_real), to_hex_string(b2_i));
    println!();

    println!("With these parameters, e returns a element in the multiplicative group Gt with the same order as G2");
    println!();

    let (g, g2):(G1, G2) = get_generator_pair();
    println!("g ∈ G1 (generator) = {:?}", g);
    println!("g2 ∈ G2 (generator) = {:?}", g2);

    // TODO: Figure out how to print elements of type Gt
//    println!("\te(g, g2) ∈ Gt (generator) = {:?}", pairing(g, g2));
    println!("Then, we can compute e(g, g2) ∈ Gt (generator)");
    println!();
    println!();
    
    
    
    /* ------------------------------------------------------------------------------
     *                                  GenRA                                       
     * ------------------------------------------------------------------------------
     */

    // Instantiate new Registration Authority
    println!("Generating signature-verification key pair (x, vk_RA) for Registration Authority (RA)...");
    let mut ra:RegistrationAuthority = RegistrationAuthority::new(g, g2);
    println!("sk_RA = x ∈ ℤ_q = (secret signature key)");
    println!("vk_RA.u ∈ G1 = {:?}", ra.vk.u);
    println!("vk_RA.v ∈ G1 = {:?}", ra.vk.v);
    println!("vk_RA.h ∈ G1 = {:?}", ra.vk.h);
    println!();


    
    /* ------------------------------------------------------------------------------
     *                                  GenSA                                       
     * ------------------------------------------------------------------------------
     */

    // Instantiate new Survey Authority
    println!("Generating signature-verification key pair (y, vk_SA) for Survey Authority (SA)...");
    let mut sa:User = SurveyAuthority::new(g, g2); 
    println!("sk_SA = y ∈ ℤ_q = (secret signature key)");
    println!("vk_SA.u ∈ G1 = {:?}", sa.vk.u);
    println!("vk_SA.v ∈ G1 = {:?}", sa.vk.v);
    println!("vk_SA.h ∈ G1 = {:?}", sa.vk.h);
    println!();
    

    /* ------------------------------------------------------------------------------
     *                                  ***NOTE***                                  
     * The setup of every exchange between the users is NOT supposed to go
     * through a central or third party like it is here. This was done only as a
     * proof-of-concept and would likely VIOLATE ANONYMITY in production code.
     * A proper implementation of ANONIZE should (at least) establish private 
     * connections between all users, and ESPECIALLY an anonymous connection
     * between +
     * ------------------------------------------------------------------------------
    */
    
    // Initialize 5 users in the userbase and register their ID with the RA
    let mut userbase:Vec<User> = Vec::new();
    for _ in 0..5 {
        let mut new_user = User::new();
        new_user.reg_user(&mut ra);
        userbase.push(new_user);
    }
    // Just for fun, some users will change their identities
    userbase[0].re_identify(&mut ra);
    userbase[3].re_identify(&mut ra);

    println!("List of registered users:");
    for id in &ra.userid_list { 
        println!("User id ∈ ℤ_q : {:?}", *id);
    }
    println!();

    /* ------------------------------------------------------------------------------
     *                                  GenSurvey                                       
     * ------------------------------------------------------------------------------
     */
    // Could theoretically choose a list of any ids, even for users who have not yet registered with
    // the RA.
    let rng = &mut rand::thread_rng();
    let unregistered_userid = Fr::random(rng);
    let mut part_list:Vec<Fr> = ra.userid_list.clone();
    println!("Unregistered user with id ∈ ℤ_q : {:?}", unregistered_userid);
    part_list.push(unregistered_userid);
    println!();

    println!("SA: Generating survey signatures for {} potential users...", part_list.len());
    let (vid, signatures):(Fr, Vec<(Fr, G1, G2)>) = sa.gen_survey(&part_list, g, g2, &ra.vk).expect("SA survey creation failed!");
    println!("Ad-hoc survey generated:");
    println!("\tvid ∈ ℤ_q (survey ID) = {:?}", vid);
    println!("\tList of authorized users:");
    for (id, sigma_1, sigma_2) in &signatures {
        println!("\t\tParticipant id:\t{:?}", *id);
        println!();
        println!("\t\t\t(σ1, σ2) ∈ G1 × G2 (SA signature for participant) = ({:?}, {:?})", *sigma_1, *sigma_2);
        print!("\t\t\tAuthorized... ");
        match authorized(*id, vid, &signatures, &sa.vk, &ra.vk, g2) {
            true    => println!("\u{2713}"),    // Checkmark    (yes!)
            false   => println!("\u{2717}")     // X mark       (no!)
        }
        println!();
    }
    println!();


    // TODO: Have all users run on separate threads for efficiency

    println!();
}


// Anyone can test if a user is authorized to take a survey
fn authorized(id:Fr, vid:Fr, Lvid:&Vec<(Fr, G1, G2)>, vk_sa:&VerificationKey, vk_ra:&VerificationKey, g2:G2) -> bool {
    
    // Search through list of participant signature to find the one corresponding to id
    for (part_id, sigma_1, sigma_2) in Lvid {
        if *part_id == id {
            return pairing(*sigma_1, g2) == ( (*vk_sa).pk * pairing((*vk_sa).u * vid + (*vk_sa).v * id + (*vk_ra).h, *sigma_2) );
        }
    }
    false
}



/*
 * Unit tests
 */

// Fuzzy test for if we have a good generator for pairing-based crypto
#[test]
fn test_generators() {

    let (g, g2):(G1, G2) = get_generator_pair();
    
    // Try 5 different random values to see if assertion holds each time
    // For random a and b, asserts that e(g^a, g_2^b) = e(g,g_2)^{ab} (RHS is generator for Gt)
    let rng = &mut rand::thread_rng();
    for _ in 0..5 {
        let a = Fr::random(rng);
        let b = Fr::random(rng);
        assert!( pairing(g * a, g2 * b) == pairing(g, g2).pow(a * b) );
    }
}

// TODO: Test U256 -> hex conversions


/*
 * Integration tests
 */


/*
 * Benchmark tests
 */

#[test]
#[allow(non_snake_case)]
// Test GenSurvey for 30 users to get mean and standard deviation
fn bench_30_user_gen_survey() {

    use std::time::{Duration, Instant};

    // Setup 
    let rng = &mut rand::thread_rng();
    let (g, g2):(G1, G2) = get_generator_pair();

    let mut ra = RegistrationAuthority::new(g, g2);
    let mut sa:User = SurveyAuthority::new(g, g2);
    const NUM_USERS:usize = 30;
    assert!(NUM_USERS > 1);
    let mut userids:Vec<Fr> = Vec::new();
    for _ in 0..NUM_USERS {
        // Skip registering user -- we only care about user ids for generating survey
        userids.push(Fr::random(rng));
    }

    // 30-participant survey for GenSurvey
    println!("GenSurvey Benchmark Test ({} users)", NUM_USERS);
    let mut sum:Duration = Duration::new(0,0);
    let mut durs:[Duration;NUM_USERS] = [Duration::new(0,0);NUM_USERS];
    for i in 0..NUM_USERS {
        let start = Instant::now();
        // One user at a time
        let _ = sa.gen_survey(&vec![userids[i]], g, g2, &ra.vk).expect("SA survey creation failed!");
        durs[i] = start.elapsed();
        sum += durs[i];
        println!("User {}: {:?}", i+1, durs[i]);
    }
    println!();
    // Calculate mean
    let mean = sum / (NUM_USERS as u32);
    // Calculate standard deviation
    let mut sum_of_diff:f32 = 0.0;
    for i in 0..NUM_USERS {
        sum_of_diff += f32::powf((((durs[i].as_millis() as i128) - (mean.as_millis() as i128)) as f32)/1000.0, 2.0);
    }
    let sd = ( sum_of_diff / ((NUM_USERS as f32)- 1.0)).sqrt();
 
    println!("Mean:\t\t{:?}", mean);
    println!("Std Dev:\t{:?}s", sd);
    println!("Total:\t\t{:?}", sum);
}


#[test]
#[ignore]
#[allow(non_snake_case)]
// Test GenSurvey for 300 users to get mean and standard deviation
fn bench_300_user_gen_survey() {

    use std::time::{Duration, Instant};

    // Setup 
    let rng = &mut rand::thread_rng();
    let (g, g2):(G1, G2) = get_generator_pair();

    let mut ra = RegistrationAuthority::new(g, g2);
    let mut sa:User = SurveyAuthority::new(g, g2);
    const NUM_USERS:usize = 300;
    assert!(NUM_USERS > 1);
    let mut userids:Vec<Fr> = Vec::new();
    for _ in 0..NUM_USERS {
        // Skip registering user -- we only care about user ids for generating survey
        userids.push(Fr::random(rng));
    }
 
    // 300-participant survey for GenSurvey
    println!("GenSurvey Benchmark Test ({} users)", NUM_USERS);
    let mut sum:Duration = Duration::new(0,0);
    let mut durs:[Duration;NUM_USERS] = [Duration::new(0,0);NUM_USERS];
    for i in 0..NUM_USERS {
        let start = Instant::now();
        // One user at a time
        let _ = sa.gen_survey(&vec![userids[i]], g, g2, &ra.vk).expect("SA survey creation failed!");
        durs[i] = start.elapsed();
        sum += durs[i];
        println!("User {}: {:?}", i+1, durs[i]);
    }
    println!();
    // Calculate mean
    let mean = sum / (NUM_USERS as u32);
    // Calculate standard deviation
    let mut sum_of_diff:f32 = 0.0;
    for i in 0..NUM_USERS {
        sum_of_diff += f32::powf((((durs[i].as_millis() as i128) - (mean.as_millis() as i128)) as f32)/1000.0, 2.0);
    }
    let sd = ( sum_of_diff / ((NUM_USERS as f32)- 1.0)).sqrt();
 
    println!("Mean:\t\t{:?}", mean);
    println!("Std Dev:\t{:?}s", sd);
    println!("Total:\t\t{:?}", sum);
}



#[test]
#[allow(non_snake_case)]
// Test Authorized for 30 users to get mean and standard deviation
fn bench_30_user_authorized() {

    use std::time::{Duration, Instant};

    // Setup 
    let rng = &mut rand::thread_rng();
    let (g, g2):(G1, G2) = get_generator_pair();

    let mut ra = RegistrationAuthority::new(g, g2);
    let mut sa:User = SurveyAuthority::new(g, g2);
    const NUM_USERS:usize = 30;
    assert!(NUM_USERS > 1);
    let mut userids:Vec<Fr> = Vec::new();
    for _ in 0..NUM_USERS {
        // Skip registering user -- we only care about user ids for generating survey
        userids.push(Fr::random(rng));
    }

    // 30-participant survey for GenSurvey
    println!("Generating {} survey signatures...", userids.len());
    let (vid, signatures):(Fr, Vec<(Fr, G1, G2)>) = sa.gen_survey(&userids, g, g2, &ra.vk).expect("SA survey creation failed!");
 
    // Check authorized for each user
    println!("User Authorized Benchmark Test ({} users)", NUM_USERS);
    let mut sum:Duration = Duration::new(0,0);
    let mut durs:[Duration;NUM_USERS] = [Duration::new(0,0);NUM_USERS];
    let _ = sa.gen_survey(&userids, g, g2, &ra.vk).expect("SA survey creation failed!");
    
    for i in 0..NUM_USERS {
        let start = Instant::now();
        // One user at a time
        let _ = authorized(userids[i], vid, &signatures, &sa.vk, &ra.vk, g2);
        durs[i] = start.elapsed();
        sum += durs[i];
        println!("User {}: {:?}", i+1, durs[i]);
    }
    println!();
    // Calculate mean
    let mean = sum / (NUM_USERS as u32);
    // Calculate standard deviation
    let mut sum_of_diff:f32 = 0.0;
    for i in 0..NUM_USERS {
        sum_of_diff += f32::powf((((durs[i].as_millis() as i128) - (mean.as_millis() as i128)) as f32)/1000.0, 2.0);
    }
    let sd = ( sum_of_diff / ((NUM_USERS as f32)- 1.0)).sqrt();
 
    println!("Mean:\t\t{:?}", mean);
    println!("Std Dev:\t{:?}s", sd);
    println!("Total:\t\t{:?}", sum);
}



#[test]
#[ignore]
#[allow(non_snake_case)]
// Test Authorized for 300 users to get mean and standard deviation
fn bench_300_user_authorized() {

    use std::time::{Duration, Instant};

    // Setup 
    let rng = &mut rand::thread_rng();
    let (g, g2):(G1, G2) = get_generator_pair();

    let mut ra = RegistrationAuthority::new(g, g2);
    let mut sa:User = SurveyAuthority::new(g, g2);
    const NUM_USERS:usize = 300;
    assert!(NUM_USERS > 1);
    let mut userids:Vec<Fr> = Vec::new();
    for _ in 0..NUM_USERS {
        // Skip registering user -- we only care about user ids for generating survey
        userids.push(Fr::random(rng));
    }

    // 300-participant survey for GenSurvey
    println!("Generating {} survey signatures...", userids.len());
    let (vid, signatures):(Fr, Vec<(Fr, G1, G2)>) = sa.gen_survey(&userids, g, g2, &ra.vk).expect("SA survey creation failed!");
 
    // Check authorized for each user
    println!("User Authorized Benchmark Test ({} users)", NUM_USERS);
    let mut sum:Duration = Duration::new(0,0);
    let mut durs:[Duration;NUM_USERS] = [Duration::new(0,0);NUM_USERS];
    let _ = sa.gen_survey(&userids, g, g2, &ra.vk).expect("SA survey creation failed!");
    
    for i in 0..NUM_USERS {
        let start = Instant::now();
        // One user at a time
        let _ = authorized(userids[i], vid, &signatures, &sa.vk, &ra.vk, g2);
        durs[i] = start.elapsed();
        sum += durs[i];
        println!("User {}: {:?}", i+1, durs[i]);
    }
    println!();
    // Calculate mean
    let mean = sum / (NUM_USERS as u32);
    // Calculate standard deviation
    let mut sum_of_diff:f32 = 0.0;
    for i in 0..NUM_USERS {
        sum_of_diff += f32::powf((((durs[i].as_millis() as i128) - (mean.as_millis() as i128)) as f32)/1000.0, 2.0);
    }
    let sd = ( sum_of_diff / ((NUM_USERS as f32)- 1.0)).sqrt();
 
    println!("Mean:\t\t{:?}", mean);
    println!("Std Dev:\t{:?}s", sd);
    println!("Total:\t\t{:?}", sum);
}
