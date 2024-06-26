
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Set;
import java.util.stream.Stream;

public class day12 {
    private static Map<String, Set<String>> graph;

    private static void add_edge(String[] names) {
        if (!graph.containsKey(names[0]))
            graph.put(names[0], new HashSet<>());
        graph.get(names[0]).add(names[1]);
        
        if (!graph.containsKey(names[1]))
            graph.put(names[1], new HashSet<>());
        graph.get(names[1]).add(names[0]);
    }

    private static class SearchToken {
        public String here;
        public Set<String> visited;
        
        public SearchToken(String here, Set<String> old_visited) {
            visited = new HashSet<>(old_visited);
            this.here = here;
            if (Character.isLowerCase(here.charAt(0)))
                visited.add(here);
        }

        public Stream<String> neighbours() {
            return graph.get(here).stream().filter(x -> !visited.contains(x));
        }
    }

    public static void main(String[] args) {
        try {
            graph = new HashMap<>();
            Files
                .lines(Paths.get("2021/day12/day12.txt"))
                .forEach(x -> add_edge(x.split("-")));

            LinkedList<SearchToken> search_stack = new LinkedList<>();
            search_stack.push(new SearchToken("start", new HashSet<>()));
            int result = 0;
            while (!search_stack.isEmpty()) {
                SearchToken here = search_stack.pop();
                if (here.here.equals("end")) {
                    result++;
                    continue;
                }
                here.neighbours().forEach(loc -> search_stack.push(new SearchToken(loc, here.visited)));
            }
            System.out.println(result);

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
