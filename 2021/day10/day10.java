
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class day10 {
    private static final Map<Character, Character> closing_pair = new HashMap<>() {{
        put(')', '(');
        put(']', '[');
        put('}', '{');
        put('>', '<');
    }};
    private static final Set<Character> openers = new HashSet<>(closing_pair.values());
    private static final Set<Character> closers = new HashSet<>(closing_pair.keySet());
    private static final Map<Character, Integer> closing_score = new HashMap<>() {{
        put(')', 3);
        put(']', 57);
        put('}', 1197);
        put('>', 25137);
    }};

    public static int score_corrupt(String input) {
        Deque<Character> stack = new LinkedList<>();
        for (char inp : input.toCharArray()) {
            if (openers.contains(inp)) {
                stack.push(inp);
            } else if (stack.peek().equals(closing_pair.get(inp))) {
                stack.pop();
            } else {
                return closing_score.get(inp);
            }
        }
        return 0;
    }

    public static void main(String[] args) {
        try {
            List<String> data = Files
                    .lines(Paths.get("2021/day10/day10.txt"))
                    .toList();
            
            System.out.println(data.stream().mapToInt(x -> score_corrupt(x)).sum());
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
