use sprs::TriMatBase;
use optrs::{self,
            assert_vec_approx_eq,
            ProblemMilp,
            ProblemMilpBase,
            Solver,
            SolverCbcCmd};

fn main () {

    println!("optrs example MILP problem and solution");

    // Sample problem 
    // min        -x0 - x1 
    // subject to -2*x0 +  2*x1 + x2 == 1
    //            -8*x0 + 10*x1 + x3 ==  13
    //            x2 <= 0
    //            x3 >= 0
    //            x0 integer
    //            x1 integer

    let mut p = ProblemMilp::new(
        vec![-1.,-1., 0., 0.],
        TriMatBase::from_triplets(
            (2, 4),
            vec![0,0,0,1,1,1],
            vec![0,1,2,0,1,3],
            vec![-2.,2.,1.,-8.,10.,1.]),
        vec![1.,13.],
        vec![-1e8,-1e8,-1e8,0.],
        vec![1e8,1e8,0.,1e8],
        vec![true, true, false, false],
    );

    let x = vec![0.5, 2., 1., 2.];

    optrs::ProblemBase::evaluate(&mut p, &x);
    
    println!("x = {:?}", p.x());
    println!("phi = {}", optrs::ProblemBase::phi(&p));
    println!("gphi = {:?}", optrs::ProblemBase::gphi(&p));
    println!("c = {:?}", p.c());
    println!("a = {:?}", p.a());
    println!("b = {:?}", p.b());
    println!("l = {:?}", p.l());
    println!("u = {:?}", p.u());
    println!("p = {:?}", p.p());

    let mut s = SolverCbcCmd::new();
    s.solve(p).unwrap();

    println!("solver status = {}", s.status());
    println!("solution = {:?}", s.solution());

    assert!(s.status().is_solved());
    assert!(s.solution().is_some());
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().x, 
                          &vec![1., 2., -1., 1.0], 
                          epsilon=1e-8);
    
}