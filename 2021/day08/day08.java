
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collector;
import java.util.stream.Collectors;

public class day08 {
    public static int decode(String encoded) {
        List<Set<Character>> samples = Arrays.stream(encoded.split(" \\| ")[0].split(" "))
            .map(x -> x.chars().mapToObj(y -> (char) y).collect(Collectors.toSet())).collect(Collectors.toList());

        Map<Integer, Set<Character>> identifieds = new HashMap<>();
        samples.sort((x,y) -> x.size() - y.size());
        identifieds.put(1, samples.get(0));
        identifieds.put(7, samples.get(1));
        identifieds.put(4, samples.get(2));
        identifieds.put(8, samples.get(9));

        identifieds.put(9, samples.stream().filter(
            x -> x.size() == 6 && x.containsAll(identifieds.get(4))
        ).toList().get(0));
        identifieds.put(0, samples.stream().filter(
            x -> x.size() == 6 && !x.containsAll(identifieds.get(4)) && x.containsAll(identifieds.get(1))
        ).toList().get(0));
        identifieds.put(6, samples.stream().filter(
            x -> x.size() == 6 && !x.containsAll(identifieds.get(1)) && !x.containsAll(identifieds.get(4))
        ).toList().get(0));
        identifieds.put(3, samples.stream().filter(
            x -> x.size() == 5 && x.containsAll(identifieds.get(1))
        ).toList().get(0));
        identifieds.put(5, samples.stream().filter(
            x -> x.size() == 5 && identifieds.get(6).containsAll(x)
        ).toList().get(0));
        identifieds.put(2, samples.stream().filter(
            x -> x.size() == 5 && !x.containsAll(identifieds.get(1)) && !identifieds.get(6).containsAll(x)
        ).toList().get(0));

        Map<Set<Character>, String> decoder = identifieds.entrySet().stream().collect(Collectors.toMap(Map.Entry::getValue, x -> x.getKey().toString()));
        return Integer.parseInt(Arrays.stream(encoded.split(" \\| ")[1].split(" "))
            .map(x -> decoder.get(x.chars().mapToObj(y -> (char) y).collect(Collectors.toSet())))
            .collect(Collector.of(StringBuilder::new, StringBuilder::append, StringBuilder::append, StringBuilder::toString))
        );
    }

    public static void main(String[] args) {
        try {
            List<String> data = Files
                    .lines(Paths.get("2021/day08/day08.txt"))
                    .toList();
            
            System.out.println(data.stream()
                .flatMap(x -> Arrays.stream(x.split(" \\| ")[1].split(" ")))
                .mapToInt(x -> {
                    switch (x.length()) {
                        case 2: case 3: case 4: case 7: return 1;
                        default: return 0;
                    }
                }).sum());
            
            System.out.println(data.stream()
                .mapToInt(x -> decode(x))
                .sum());
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}