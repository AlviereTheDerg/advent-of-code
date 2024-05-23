
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Predicate;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class day03 {
    public static int extract_life_support(List<String> data, Predicate<Integer> filter) {
        for (AtomicInteger index = new AtomicInteger(); index.get() < data.get(0).length(); index.incrementAndGet()) {
            char match = filter.test(data.stream().mapToInt((s) -> s.charAt(index.get()) == '1' ? 1 : -1).sum()) ? '1' : '0';
            data = data.stream().filter((s) -> s.charAt(index.get()) == match).toList();
            if (data.size() == 1) {
                return Integer.parseInt(data.get(0), 2);
            }
        }
        return 0;
    }

    public static void main(String[] args) {
        try {
            // input file -> list of Integer
            List<String> data = Files
                    .lines(Paths.get("2021/day03/day03.txt"))
                    .toList();

            List<Boolean> commons = IntStream.range(0, data.get(0).length())
                .boxed()
                .map((i) -> {
                    return (int) data.stream().mapToInt((s) -> s.charAt(i) == '1' ? 1 : -1).sum() >= 0;
                })
                .toList();
            
            int gamma    = Integer.parseInt(commons.stream().map((x) -> x ? "1" : "0").collect(Collectors.joining()), 2);
            int epsilon  = Integer.parseInt(commons.stream().map((x) -> x ? "0" : "1").collect(Collectors.joining()), 2);
            System.out.println(gamma * epsilon);

            System.out.println(extract_life_support(data, x -> x >= 0) * extract_life_support(data, x -> x < 0));

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
