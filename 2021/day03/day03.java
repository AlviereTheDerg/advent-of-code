
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class day03 {
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

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
