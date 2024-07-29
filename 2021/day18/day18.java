
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class day18 {
    private static class SnailfishNumber {
        public boolean is_simple = true;
        public Integer simple_number;
        public SnailfishNumber left, right, parent;

        public SnailfishNumber(String text_input, SnailfishNumber parent) {
            this.parent = parent;
            // identify crucial comma, comma occurs at depth = 0 within current scope
            // [[stuff,(depth 1) stuff],(depth 0) stuff]
            int comma_position, depth = -1;
            for (comma_position = 0; comma_position < text_input.length(); comma_position++) {
                if (text_input.charAt(comma_position) == '[')
                    depth++;
                if (text_input.charAt(comma_position) == ']')
                    depth--;
                if (depth == 0 && text_input.charAt(comma_position) == ',') {
                    is_simple = false; // flag as complex
                    break;
                }
            }
            // no comma -> simple number
            if (is_simple) {
                simple_number = Integer.parseInt(text_input);
                return;
            }

            left  = new SnailfishNumber(text_input.substring(1, comma_position), this);
            right = new SnailfishNumber(text_input.substring(comma_position + 1, text_input.length() - 1), this);
        }

        public SnailfishNumber(SnailfishNumber left, SnailfishNumber right, SnailfishNumber parent) {
            is_simple = false;
            this.parent = parent;
            this.left = left;
            left.parent = this;
            this.right = right;
            right.parent = this;
        }

        public SnailfishNumber(Integer simple_number, SnailfishNumber parent) {
            this.simple_number = simple_number;
            this.parent = parent;
        }

        public void split() {
            if (!this.is_simple)
                return; // non-simple numbers can't split

            this.left = new SnailfishNumber(this.simple_number / 2, this);
            this.right = new SnailfishNumber((this.simple_number / 2) + (this.simple_number % 2), this);
            this.is_simple = false;
        }

        private SnailfishNumber get_child(boolean right) {
            return right ? this.right : this.left;
        }

        private SnailfishNumber step_directional(boolean right) {
            // comments explain for stepping left, direction inverted for stepping right
            // ascend until a point where this is the right child of its parent
            SnailfishNumber here = this;
            while (here.parent != null && here.parent.get_child(right) == here)
                here = here.parent;

            // if this is leftmost leaf, return (there is no leaf in that direction)
            if (here.parent == null || here.parent.get_child(right) == here)
                return null; 

            // go down to the left by one
            here = here.parent.get_child(right);
            // descend down to the right until reaching a simple node
            while (!here.is_simple)
                here = here.get_child(!right);
            return here;
        }

        public void send_directional(int number_to_send, boolean right) {
            SnailfishNumber here = this.step_directional(right);
            if (here != null) // if there is a leaf in that direction, update its value
                here.simple_number += number_to_send;
        }

        public void explode() {
            if (this.is_simple)
                return; //simple numbers can't explode
            
            this.send_directional(this.left.simple_number, false);
            this.send_directional(this.right.simple_number, true);
            this.left = null;
            this.right = null;
            this.is_simple = true;
            this.simple_number = 0;
        }

        private boolean internal_reduce_explode() {
            int explosion_depth = 4;
            SnailfishNumber here = this;
            while (!here.is_simple)
                here = here.left; // get leftmost leaf (simple node)
            
            while (here != null) {
                // check if this node's parent is a pair
                SnailfishNumber parent = here.parent;
                if (parent != null && parent.left.is_simple && parent.right.is_simple) {
                    // check if it needs to explode
                    int depth = 0;
                    for (SnailfishNumber chain = parent; chain != null; chain = chain.parent)
                        depth++;
                    if (depth > explosion_depth) {
                        parent.explode();
                        return true;
                    }
                }

                // go to next leaf
                here = here.step_directional(true);
            }
            return false;
        }

        private boolean internal_reduce_split() {
            int split_size = 10;
            SnailfishNumber here = this;
            while (!here.is_simple)
                here = here.left; // get leftmost leaf (simple node)
            
            while (here != null) {
                // check if this node needs to split
                if (here.simple_number >= split_size) {
                    here.split();
                    return true;
                }

                here = here.step_directional(true);
            }
            return false;
        }

        public void reduce() {
            while (true) {
                if (this.internal_reduce_explode()) continue;
                if (this.internal_reduce_split()) continue;
                break;
            }
        }

        public static SnailfishNumber add(SnailfishNumber A, SnailfishNumber B) {
            A = new SnailfishNumber(A.toString(), null);
            B = new SnailfishNumber(B.toString(), null);
            SnailfishNumber result = new SnailfishNumber(A, B, null);
            result.reduce();
            return result;
        }

        public String toString() {
            if (is_simple)
                return simple_number.toString();
            return String.format("[%s,%s]", left.toString(), right.toString());
        }

        public int magnitude() {
            if (this.is_simple)
                return this.simple_number;
            return 3 * left.magnitude() + 2 * right.magnitude();
        }
    }

    public static void main(String[] args) {
        try {
            List<SnailfishNumber> data = Files
                    .lines(Paths.get("2021/day18/day18.txt"))
                    .map(x -> new SnailfishNumber(x, null))
                    .toList();
            
            SnailfishNumber result = null;
            for (SnailfishNumber entry : data)
                if (result == null)
                    result = entry;
                else
                    result = SnailfishNumber.add(result, entry);
            System.out.println(result.magnitude());

            int largest = 0;
            for (int i = 0; i < data.size(); i++) {
                for (int j = 0; j < data.size(); j++) {
                    if (i == j) continue;
                    largest = Math.max(largest, SnailfishNumber.add(
                        data.get(i), 
                        data.get(j)
                    ).magnitude());
                }
            }
            System.out.println(largest);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
