use std::cmp;
use std::collections::HashMap;
use euler::bigint;

/*
rec fn with starting point and cache
- check base case
- check cache
- get result of rec fn
- set cache
- return result
*/
pub fn fib_rec(i: i64, cache: &mut HashMap<i64, i128>) -> i128 {
    // base cases
    if i == 0 { return 0; }
    if i == 1 { return 1; }
    // check cache
    if cache.contains_key(&i) {
        return *cache.get(&i).unwrap();
    }
    let result = fib_rec(i-2, cache) + fib_rec(i-1, cache);
    // set cache
    cache.insert(i, result);
    result
}


/*
What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
*/
#[allow(dead_code)]
pub fn problem_11() -> i64 {
    let str = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";
    let tmp_str = str.replace("\n", " ");
    let grid_str: Vec<&str> = tmp_str.split(" ").collect();
    let mut grid: Vec<i64> = vec!();
    for g in grid_str.iter() {
        grid.push(g.parse().unwrap());
    }

    let nrow: usize = 20;
    let sweep_down = |i: usize| -> i64 {  grid[i] * grid[i+nrow] * grid[i+nrow*2] * grid[i+nrow*3] };
    let sweep_right = |i: usize| -> i64 {  grid[i] * grid[i+1] * grid[i+2] * grid[i+3] };
    let sweep_diag_right = |i: usize| -> i64 {  grid[i] * grid[i+nrow+1] * grid[i+nrow*2+2] * grid[i+nrow*3+3] };
    let sweep_diag_left = |i: usize| -> i64 {  grid[i] * grid[i+nrow-1]  * grid[i+nrow*2-2] * grid[i+nrow*3-3] };
    let mut max = 0;

    for i in 0..(nrow * (nrow-3)) {
        let row_down = sweep_down(i);
        max = cmp::max(max, row_down);
    }

    for i in 0..(nrow * (nrow - 3)) {
        let j = (i / (nrow - 3)) * nrow + (i % (nrow-3));
        let row_right = sweep_right(j);
        max = cmp::max(max, row_right);
    }

    for i in 0..((nrow-3) * (nrow - 3)) {
        let j = (i / (nrow - 3)) * (nrow-3) + (i % (nrow-3));
        let diag_right = sweep_diag_right(j);
        max = cmp::max(max, diag_right);
    }

    for i in 0..((nrow-3) * (nrow - 3)) {
        let j = (i / (nrow - 3)) * (nrow-3) + (i % (nrow-3)) + 3;
        let diag_left = sweep_diag_left(j);
        max = cmp::max(max, diag_left);
    }
    max
}



/*
What is the value of the first triangle number to have over five hundred divisors?
*/
#[allow(dead_code)]
pub fn problem_12() -> i64 {
    let n = 500;
    let mut i = 1;
    let mut sum = 0;
    let mut divisor_count;
    loop {
        sum += i;
        divisor_count = 0;
        let mut t = sum;
        for k in 1..sum {
            if k >= t { break }
            if sum % k == 0 {
                if k == sum / k { // perfect square
                    divisor_count += 1;
                } else {
                    divisor_count += 2;
                }
                t = sum / k;
            }
        }
        if divisor_count >= n { break };
        i += 1;
    }
    sum
}


/*
Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
*/
pub fn problem_13() -> i64 {
    let numbers_str = "37107287533902102798797998220837590246510135740250
46376937677490009712648124896970078050417018260538
74324986199524741059474233309513058123726617309629
91942213363574161572522430563301811072406154908250
23067588207539346171171980310421047513778063246676
89261670696623633820136378418383684178734361726757
28112879812849979408065481931592621691275889832738
44274228917432520321923589422876796487670272189318
47451445736001306439091167216856844588711603153276
70386486105843025439939619828917593665686757934951
62176457141856560629502157223196586755079324193331
64906352462741904929101432445813822663347944758178
92575867718337217661963751590579239728245598838407
58203565325359399008402633568948830189458628227828
80181199384826282014278194139940567587151170094390
35398664372827112653829987240784473053190104293586
86515506006295864861532075273371959191420517255829
71693888707715466499115593487603532921714970056938
54370070576826684624621495650076471787294438377604
53282654108756828443191190634694037855217779295145
36123272525000296071075082563815656710885258350721
45876576172410976447339110607218265236877223636045
17423706905851860660448207621209813287860733969412
81142660418086830619328460811191061556940512689692
51934325451728388641918047049293215058642563049483
62467221648435076201727918039944693004732956340691
15732444386908125794514089057706229429197107928209
55037687525678773091862540744969844508330393682126
18336384825330154686196124348767681297534375946515
80386287592878490201521685554828717201219257766954
78182833757993103614740356856449095527097864797581
16726320100436897842553539920931837441497806860984
48403098129077791799088218795327364475675590848030
87086987551392711854517078544161852424320693150332
59959406895756536782107074926966537676326235447210
69793950679652694742597709739166693763042633987085
41052684708299085211399427365734116182760315001271
65378607361501080857009149939512557028198746004375
35829035317434717326932123578154982629742552737307
94953759765105305946966067683156574377167401875275
88902802571733229619176668713819931811048770190271
25267680276078003013678680992525463401061632866526
36270218540497705585629946580636237993140746255962
24074486908231174977792365466257246923322810917141
91430288197103288597806669760892938638285025333403
34413065578016127815921815005561868836468420090470
23053081172816430487623791969842487255036638784583
11487696932154902810424020138335124462181441773470
63783299490636259666498587618221225225512486764533
67720186971698544312419572409913959008952310058822
95548255300263520781532296796249481641953868218774
76085327132285723110424803456124867697064507995236
37774242535411291684276865538926205024910326572967
23701913275725675285653248258265463092207058596522
29798860272258331913126375147341994889534765745501
18495701454879288984856827726077713721403798879715
38298203783031473527721580348144513491373226651381
34829543829199918180278916522431027392251122869539
40957953066405232632538044100059654939159879593635
29746152185502371307642255121183693803580388584903
41698116222072977186158236678424689157993532961922
62467957194401269043877107275048102390895523597457
23189706772547915061505504953922979530901129967519
86188088225875314529584099251203829009407770775672
11306739708304724483816533873502340845647058077308
82959174767140363198008187129011875491310547126581
97623331044818386269515456334926366572897563400500
42846280183517070527831839425882145521227251250327
55121603546981200581762165212827652751691296897789
32238195734329339946437501907836945765883352399886
75506164965184775180738168837861091527357929701337
62177842752192623401942399639168044983993173312731
32924185707147349566916674687634660915035914677504
99518671430235219628894890102423325116913619626622
73267460800591547471830798392868535206946944540724
76841822524674417161514036427982273348055556214818
97142617910342598647204516893989422179826088076852
87783646182799346313767754307809363333018982642090
10848802521674670883215120185883543223812876952786
71329612474782464538636993009049310363619763878039
62184073572399794223406235393808339651327408011116
66627891981488087797941876876144230030984490851411
60661826293682836764744779239180335110989069790714
85786944089552990653640447425576083659976645795096
66024396409905389607120198219976047599490197230297
64913982680032973156037120041377903785566085089252
16730939319872750275468906903707539413042652315011
94809377245048795150954100921645863754710598436791
78639167021187492431995700641917969777599028300699
15368713711936614952811305876380278410754449733078
40789923115535562561142322423255033685442488917353
44889911501440648020369068063960672322193204149535
41503128880339536053299340368006977710650566631954
81234880673210146739058568557934581403627822703280
82616570773948327592232845941706525094512325230608
22918802058777319719839450180888072429661980811197
77158542502016545090413245809786882778948721859617
72107838435069186155435662884062257473692284509516
20849603980134001723930671666823555245252804609722
53503534226472524250874054075591789781264330331690";
    let numbers_repl = numbers_str.replace("\n", " ");
    let grid_str: Vec<&str> = numbers_repl.split(" ").collect();
    let mut grid_base: Vec<i128> = vec!();
    let mut grid_top: Vec<i128> = vec!();
    for g in grid_str.iter() {
        grid_base.push(g[20..50].parse().unwrap());
        grid_top.push(g[0..20].parse().unwrap());
    }
    let sum_b: i128 = grid_base.iter().sum();
    let mut sum_t: i128 = grid_top.iter().sum();
    let diff = sum_b / 1_000_000_000_000_000_000_000_000_000_000;
    sum_t = sum_t + diff;
    sum_t.to_string()[0..10].parse().unwrap()
}



/*
n -> n/2 (when n is even)
n -> 3n + 1 (when n is odd)
Which starting number, under one million, produces the longest chain?
1 -> 2 -> 4 -> 8 -> 16 -> 5 -> 10 -> 20
                                \
                                3 -> 6
*/
pub fn collatz_rec(i: i64, cache: &mut HashMap<i64, i128>) -> i128 {
    if i < 2 { return 1 }
    match cache.get(&i) {
        Some(&value) => return value,
        _ => (),
    }
    let mut result;
    if i % 2 == 0 {
        result = collatz_rec(i/2, cache) + 1;
    } else {
        result = collatz_rec(3*i + 1, cache) + 1;
    }
    cache.insert(i, result);
    result
}


pub fn problem_14() -> i64 {
    let n = 1_000_000;
    let mut cache = HashMap::new();
    for i in 0..n {
        let f = collatz_rec(i, &mut cache);
    }
    let mut max_value = 0;
    let mut max_key = 0;
    for (&k, &v) in cache.iter() {
        if v > max_value {
            max_value = v;
            max_key = k;
        }
    }
    max_key
}


type AdjList = HashMap<i128, Vec<i128>>;
/*
Only able to move right and down to the bottom right corner.
How many such routes are there through a 20×20 grid?
- its a dag
1 -> 2 -> 3
|    |    |
v    v    v
4 -> 5 -> 6
|    |    |
v    v    v
7 -> 8 -> 9

the solution is to backtrack the count,
you know
f(1) = 1
f(2) = f(1)
f(5) = f(4) + f(5)
f(9) = f(6) + f(8)
*/
pub fn create_graph_p15(n: i128) -> AdjList {
    let total = n*n;
    let mut outgoing: AdjList = HashMap::new();
    for i in 1..(total+1) {
        outgoing.insert(i, Vec::new());
        if i < 1 { continue }
        if i % n != 0 {
            let m = i + 1;
            outgoing.entry(i).or_insert(Vec::new()).push(m);
        }
        // not on last row
        if i < (n * (n-1) + 1) {
            let m = i + n;
            outgoing.entry(i).or_insert(Vec::new()).push(m);
        }
    }
    outgoing
}

pub fn reverse_edges(edges: &mut AdjList) -> AdjList {
    let mut in_edges: AdjList = HashMap::new();
    for (key, value) in edges.iter() {
        for i in value.iter() {
            in_edges.entry(*i).or_insert(Vec::new()).push(*key);
        }
    }
    in_edges
}

pub fn count_paths(edges: &AdjList, n: i128) -> i128 {
    let mut npaths = HashMap::new();
    // base case
    npaths.insert(1, 1);
    for i in 2..(n*n+1) {
        let count_iter = edges.get(&i).unwrap().iter();
        let mut count = 0;
        for k in count_iter {
            count += npaths.get(k).unwrap();
        }
        npaths.insert(i, count);
    }
    let index = n*n;
    *npaths.get(&index).unwrap()
}

pub fn problem_15() -> i128 {
    let n = 20+1;
    let mut outgoing = create_graph_p15(n);
    // reverse as incoming edges
    let incoming = reverse_edges(&mut outgoing);
    let npaths = count_paths(&incoming, n);
    npaths
}


pub fn problem_16() -> i128 {
    let x = bigint::big_exponential(2, 1000);
    let y = x.value();
    let mut accum = 0;
    for i in 0..y.len() {
        let ch = y.chars().nth(i).unwrap();
        accum += ch.to_digit(10).unwrap();
    }
    println!("total: {}", accum);
    println!("y: {:?}", y);
    accum as i128
}


/*
one, two, three, four, five, six, seven, eight, nine,
ten, eleven, twelve, thirteen, fourteen, fifteen, sixteen, seventeen, eighteen. nineteen,
ten, twenty, thirty, forty, fifty, sixty, seventy, eighty, ninety,
one hundred and one,
two hundred,
one thousand
*/
pub fn problem_17() -> i128 {
    let t_1_9 = [3, 3, 5, 4, 4, 3, 5, 5, 4];
    let t_10_19 = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
    let sum_t_1_9 = t_1_9.iter().fold(0, |a, &b| a + b);
    let sum_t_10_19 = t_10_19.iter().fold(0, |a, &b| a + b);

    // tens, twenty, ..., ninety
    let tens = [0, 6, 6, 5, 5, 5, 7, 6, 6];
    // t_1_20 = sum(1-20)
    // t_20_100 = [6, 6, 5, 5, 5, 7, 6, 6] * 10 + sum(1-9) * 8
    let mut t_1_99 = 0;
    for i in tens.iter() {
        t_1_99 += i * 10 as i128;
    }
    t_1_99 += sum_t_1_9 * 9 + sum_t_10_19;
    /*
    100-199
    t_100_199 = [3+7] * 100 + t_1_99
    */
    let mut t_100_999 = 0;
    for i in t_1_9.iter() {
        t_100_999 += (*i as i128 + 7 + 3) * 100 + t_1_99 - 3;
    }
    let t = t_1_99 + t_100_999 + 11;
    t
}
