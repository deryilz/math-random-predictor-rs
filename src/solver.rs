use z3::ast::*;
use z3::{Config, Context, Solver};

/// Takes 5+ Math.random() values and returns the next
pub fn predict_math_random(nums: &[f64]) -> Option<f64> {
    if nums.len() < 5 {
        return None;
    }

    let ctx = Context::new(&Config::default());
    let solver = Solver::new(&ctx);

    let mut se_state0 = BV::new_const(&ctx, "se_state0", 64);
    let mut se_state1 = BV::new_const(&ctx, "se_state1", 64);

    let bv_int = |i| BV::from_u64(&ctx, i, 64);

    for num in nums.iter().cloned().rev() {
        let s1 = se_state0.clone();
        let s0 = se_state1.clone();

        se_state0 = s0.clone();

        let s1 = s1.bvxor(&s1.bvshl(&bv_int(23)));
        let s1 = s1.bvxor(&s1.bvlshr(&bv_int(17)));
        let s1 = s1.bvxor(&s0);
        let s1 = s1.bvxor(&s0.bvlshr(&bv_int(26)));

        se_state1 = s1.clone();

        let as_long = (num + 1.0).to_bits();
        let ideal_mantissa = Int::from_u64(&ctx, as_long & 0xFFFFFFFFFFFFF);
        let real_mantissa = se_state0.bvlshr(&bv_int(12)).to_int(true);

        solver.assert(&ideal_mantissa._eq(&real_mantissa));
    }

    solver.check();

    solver.get_model().and_then(|model| {
        let computed_se_state0 = model
            .iter()
            .find(|decl| decl.name() == "se_state0")?
            .apply(&[])
            .as_bv()?;
        let as_long = model.eval(&computed_se_state0, true)?.as_u64()?;

        let masked = (as_long >> 12) | 0x3FF0000000000000;
        let as_float: f64 = unsafe { std::mem::transmute(masked) };

        Some(as_float - 1.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let nums = vec![
            0.27890503818404655,
            0.476761381535326,
            0.5803780155127019,
            0.9587321411556831,
            0.14578119138062928,
        ];
        assert_eq!(predict_math_random(&nums), Some(0.9693722911582865));

        let nums = vec![
            0.50,
            0.476761381535326,
            0.5803780155127019,
            0.9587321411556831,
            0.14578119138062928,
        ];
        assert_eq!(predict_math_random(&nums), None);

        let nums = vec![0.27890503818404655];
        assert_eq!(predict_math_random(&nums), None);

        let nums = vec![
            0.9311600617849973,
            0.3551442693830502,
            0.7923158995678377,
            0.787777942408997,
            0.376372264303491,
        ];
        assert_eq!(predict_math_random(&nums), Some(0.23137147109312428));
    }
}
