/*
Solve a given equation and return the value of 'x' in the form of a string "x=#value". The equation contains only '+', '-' operation, the variable 'x' and its coefficient. You should return "No solution" if there is no solution for the equation, or "Infinite solutions" if there are infinite solutions for the equation.

If there is exactly one solution for the equation, we ensure that the value of 'x' is an integer.



Example 1:

Input: equation = "x+5-3+x=6+x-2"
Output: "x=2"
Example 2:

Input: equation = "x=x"
Output: "Infinite solutions"
Example 3:

Input: equation = "2x=x"
Output: "x=0"


Constraints:

3 <= equation.length <= 1000
equation has exactly one '='.
equation consists of integers with an absolute value in the range [0, 100] without any leading zeros, and the variable 'x'.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/solve-the-equation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let mut eq_tup = (0, 0);
        equation
            .replace("-", "+-")
            .split("=")
            .enumerate()
            .for_each(|(idx, half_equation)| {
                let half_equation = if half_equation.starts_with("+") { half_equation[1..].to_string() } else { half_equation.to_string() };
                half_equation.split("+").for_each(|unit| {
                    let mut fix = 1;
                    let mut unit = unit.to_string();
                    if unit.starts_with("-") {
                        fix = -1;
                        unit.remove(0);
                    }

                    if unit.ends_with("x") {
                        unit.remove(unit.len() - 1);
                        eq_tup.0 += if idx == 0 { 1 } else { -1 } * fix * match unit.parse::<i32>() {
                            Ok(n) => n,
                            Err(_) => 1,
                        }
                    } else {
                        eq_tup.1 += if idx == 0 { 1 } else { -1 } * fix * unit.parse::<i32>().unwrap();
                    }
                })
            });

        if eq_tup.0 == 0 && eq_tup.1 == 0 {
            "Infinite solutions".to_string()
        } else if eq_tup.0 == 0 && eq_tup.1 != 0 {
            "No solution".to_string()
        } else {
            format!("x={}", -1 * eq_tup.1 / eq_tup.0)
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::solve_equation("x+5-3+x=6+x-2".to_owned()), "x=2".to_owned());
    assert_eq!(Solution::solve_equation("x=x".to_owned()), "Infinite solutions".to_owned());
    assert_eq!(Solution::solve_equation("2x=x".to_owned()), "x=0".to_owned());
    assert_eq!(Solution::solve_equation("x=x+3".to_owned()), "No solution".to_owned());
    assert_eq!(Solution::solve_equation("-x=-1".to_owned()), "x=1".to_owned());
}