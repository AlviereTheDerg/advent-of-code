
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
        public int backsies;
        
        public SearchToken(String here, Set<String> old_visited, int backsies) {
            visited = new HashSet<>(old_visited);
            this.here = here;
            this.backsies = backsies;
            if (Character.isLowerCase(here.charAt(0)))
                visited.add(here);
        }

        public Stream<String> neighbours() {
            return graph.get(here).stream();
        }
    }

    private static int search(int backsies) {
        LinkedList<SearchToken> search_stack = new LinkedList<>();
        search_stack.push(new SearchToken("start", new HashSet<>(), backsies));
        int result = 0;
        while (!search_stack.isEmpty()) {
            SearchToken here = search_stack.pop();
            if (here.backsies < 0) // if a search token is invalid (checked a small room too many times)
                continue;

            if (here.here.equals("end")) {
                result++;
                continue;
            }

            here.neighbours()
                .filter(loc -> !"start".equals(loc))
                .forEach(loc -> search_stack.push(
                    new SearchToken(loc, here.visited, here.backsies - (here.visited.contains(loc) ? 1 : 0))
                )); 
                // big locations are not in visited
                // small locations added to visited upon first visit
                // backsies count reduced by 1 if trying to revisit a small location
        }
        return result;
    }

    public static void main(String[] args) {
        try {
            graph = new HashMap<>();
            Files
                .lines(Paths.get("2021/day12/day12.txt"))
                .forEach(x -> add_edge(x.split("-")));

            System.out.println(search(0));
            System.out.println(search(1));
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
