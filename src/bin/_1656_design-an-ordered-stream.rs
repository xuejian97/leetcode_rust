/*
There is a stream of n (idKey, value) pairs arriving in an arbitrary order, where idKey is an integer between 1 and n and value is a string. No two pairs have the same id.

Design a stream that returns the values in increasing order of their IDs by returning a chunk (list) of values after each insertion. The concatenation of all the chunks should result in a list of the sorted values.

Implement the OrderedStream class:

OrderedStream(int n) Constructs the stream to take n values.
String[] insert(int idKey, String value) Inserts the pair (idKey, value) into the stream, then returns the largest possible chunk of currently inserted values that appear next in the order.
 

Example:



Input
["OrderedStream", "insert", "insert", "insert", "insert", "insert"]
[[5], [3, "ccccc"], [1, "aaaaa"], [2, "bbbbb"], [5, "eeeee"], [4, "ddddd"]]
Output
[null, [], ["aaaaa"], ["bbbbb", "ccccc"], [], ["ddddd", "eeeee"]]

Explanation
// Note that the values ordered by ID is ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"].
OrderedStream os = new OrderedStream(5);
os.insert(3, "ccccc"); // Inserts (3, "ccccc"), returns [].
os.insert(1, "aaaaa"); // Inserts (1, "aaaaa"), returns ["aaaaa"].
os.insert(2, "bbbbb"); // Inserts (2, "bbbbb"), returns ["bbbbb", "ccccc"].
os.insert(5, "eeeee"); // Inserts (5, "eeeee"), returns [].
os.insert(4, "ddddd"); // Inserts (4, "ddddd"), returns ["ddddd", "eeeee"].
// Concatentating all the chunks returned:
// [] + ["aaaaa"] + ["bbbbb", "ccccc"] + [] + ["ddddd", "eeeee"] = ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"]
// The resulting order is the same as the order above.
 

Constraints:

1 <= n <= 1000
1 <= id <= n
value.length == 5
value consists only of lowercase letters.
Each call to insert will have a unique id.
Exactly n calls will be made to insert.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/design-an-ordered-stream
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

struct OrderedStream {
    stream: Vec<String>,
    ptr: i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            stream: vec!["".to_owned(); n as usize + 1],
            ptr: 1
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.stream[id_key as usize] = value;

        if self.ptr == id_key {
            let mut res = vec![];
            for i in self.ptr as usize..self.stream.len() {
                if self.stream[i] != "" {
                    res.push(self.stream[i].clone());
                } else {
                    self.ptr = i as i32;
                    break;
                }
            }
            res
        } else {
            vec![]
        }
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

fn main() {
    let mut os = OrderedStream::new(5);
    println!("{:?}", os.insert(3, "ccccc".to_owned()));
    println!("{:?}", os.insert(1, "aaaaa".to_owned()));
    println!("{:?}", os.insert(2, "bbbbb".to_owned()));
    println!("{:?}", os.insert(5, "eeeee".to_owned()));
    println!("{:?}", os.insert(4, "ddddd".to_owned()));
}