pub mod problems {
   
   //farhana 202210659
   
   /// 1에서 n까지의 정수의 합을 구하는 함수 recursive_sum을 작성하라.
    /// 
    /// 기존 n * (n + 1) / 2 공식을 사용하지 말고 재귀함수로 작성할 것.
    /// (가정) n > 0
    /// ```
    /// use rust_hw3::problems::recursive_sum;
    /// 
    /// assert_eq!(recursive_sum(50), 1275);
    /// assert_eq!(recursive_sum(100), 5050);
    /// ```
    pub fn recursive_sum(n: i32) -> i32 { //3.1
        if n == 1 { //base case
            1
        } else {
            n + recursive_sum(n - 1)
        }
    }

    /// 1에서 n까지의 정수의 곱을 구하는 함수 factorial을 작성하라.
    /// 
    /// 1 × ... × n을 재귀함수로 작성할 것.
    /// (가정) n > 0
    /// ```
    /// use rust_hw3::problems::factorial;
    ///
    /// assert_eq!(factorial(4), 24);
    /// assert_eq!(factorial(5), 120);
    /// ```
    pub fn factorial(n: i32) -> i32 { //3.2
        if n == 1 { //base
            1
        } else { //call func
            n * factorial(n-1)
        }
    }

    /// 피보나치 수열을 계산하는 함수 fib를 재귀함수로 작성하라.
    /// 
    /// 1번과 2번째 항은 1이며, 3번째 항부터는 앞 2개 항의 합으로 이루어진다.
    /// F1 = F2 = 1, Fn = Fn-2 + Fn-1을 재귀함수로 구현할 것.
    /// (가정) n > 0
    /// ```
    /// use rust_hw3::problems::fib;
    ///
    /// assert_eq!(fib(4), 3);
    /// assert_eq!(fib(5), 5);
    /// assert_eq!(fib(6), 8);
    /// ```
    pub fn fib(n: i32) -> i32 { //3.3
        if n == 1 || n == 2 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    /// 최대공약수를 구하는 함수 gcd를 재귀함수로 작성하라.
    /// 
    /// 유클리드 호제법을 이용하여 m과 n의 최대공약수를 반환한다.
    /// (가정) m > 0, n > 0
    /// ```
    /// use rust_hw3::problems::gcd;
    /// 
    /// assert_eq!(gcd(15, 20), 5);
    /// assert_eq!(gcd(1234, 123), 1);
    /// ```
    pub fn gcd(m: i32, n: i32) -> i32 { //3.4
        if n == 0 {
            m 
        } else {
            gcd(n , m % n )
        }
    }

    /// 콜라츠 추측을 재귀함수로 작성하라.
    /// 
    /// 주어진 값이 짝수인 경우 2로 나누며, 홀수인 경우 3을 곱한 뒤 1을 더한다.
    /// 값이 1이 될 때까지 반복하며, 반복한 횟수를 반환한다.
    /// ex) 5 -> 16 -> 8 -> 4 -> 2 -> 1, 총 5회
    /// (가정) val > 0
    /// ```
    /// use rust_hw3::problems::collatz;
    /// 
    /// assert_eq!(collatz(5, 0), 5);
    /// assert_eq!(collatz(17, 0), 12);
    /// ```
    pub fn collatz(val: i32, count: i32) -> i32 { //3.5
        if val == 1 {
            count
        } else if val % 2 == 0 {
            collatz(val / 2, count + 1)
        } else {
            collatz(val * 3 + 1, count + 1)
        }
    }

    /// 두 개의 정수를 더하는 Closure를 반환하는 sum_closure를 작성하라.
    /// 
    /// 두 개의 i32 파라미터를 받아 더한 뒤 반환하는 Closure를 작성하여 반환한다.
    /// ```
    /// use rust_hw3::problems::sum_closure;
    /// 
    /// assert_eq!(sum_closure()(1, 3), 4);
    /// assert_eq!(sum_closure()(10, 20), 30);
    /// ```
    pub fn sum_closure() -> impl Fn(i32, i32) -> i32 { //3.6
        |a: i32, b: i32| a + b
    }

    /// 임의의 2차 함수에 대하여 두 함수 값의 합을 구하는 함수 sum_of_fun_val를 작성하라.
    /// 
    /// f(x) = ax^2 + bx + c을 Closure로 정의한 뒤 f(d) + f(e)의 값을 반환한다.
    /// ```
    /// use rust_hw3::problems::sum_of_fun_val;
    /// 
    /// assert_eq!(sum_of_fun_val(1, 2, 1, 3, 4), 41);
    /// assert_eq!(sum_of_fun_val(1, -3, -1, 200, 123), 54158);
    /// ```
    pub fn sum_of_fun_val(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 { //3.7

    let f = |x: i32| -> i32 { a * x * x + b * x + c };
    let sum = f(d) + f(e); // f(d) and f(e), and sum values
    sum
    }

    /// 정수를 임의의 2차 함수에 세번 적용한 값을 계산하는 함수 comp3을 작성하라.
    /// 
    /// 2차 함수 f(x) = ax^2 + bx + c를 Closure로 정의한 뒤 f(f(f(d)))의 값을 반환한다.
    /// ```
    /// use rust_hw3::problems::comp3;
    /// 
    /// assert_eq!(comp3(1, 1, 1, 1), 183);
    /// assert_eq!(comp3(2, 2, 2, 2), 357014);
    /// ```
    pub fn comp3(a: i32, b: i32, c: i32, d: i32) -> i32 {//3.8
    let f = |x: i32| -> i32 { a * x * x + b * x + c };
    let result = f(f(f(d)));
    result
    }

    /// 주어진 벡터의 모든 값에 1을 더한 뒤 반환하는 함수 vector_map을 작성하라.
    /// 
    /// Iterator, Closure를 사용하여 작성한다.
    /// ```
    /// use rust_hw3::problems::vector_map;
    /// 
    /// assert_eq!(vector_map(vec![2, 3, 4, 5]), vec![3, 4, 5, 6]);
    /// assert_eq!(vector_map((0..5).collect()), vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn vector_map(vec: Vec<i32>) -> Vec<i32> { //3.9
        vec.iter().map(|x| x + 1).collect()
    }

    /// 주어진 벡터의 모든 값에 3을 더하고, 짝수만을 남긴 뒤 3을 곱한 값을 반환하라.
    /// 
    /// Iterator, Closure를 사용하여 작성한다.
    /// ```
    /// use rust_hw3::problems::vector_map_filter;
    /// 
    /// assert_eq!(vector_map_filter(vec![2, 3, 4, 5]), vec![18, 24]);
    /// assert_eq!(vector_map_filter((1..=10).collect()), vec![12, 18, 24, 30, 36]);
    /// ```
    pub fn vector_map_filter(vec: Vec<i32>) -> Vec<i32> { //3.10
        vec.iter()
        .map(|x| x + 3)              //  add 3
        .filter(|&x| x % 2 == 0)     // remove even numbers
        .map(|x| x * 3)              // multiply by 3
        .collect() 
    }
}
