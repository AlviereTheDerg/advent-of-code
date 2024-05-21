
import java.util.List;
import java.util.stream.IntStream;
import java.nio.file.Files;
import java.nio.file.Paths;

public class day01 {
    public static void main(String[] args) {
        try {
            // input file -> list of Integer
            List<Integer> data = Files
                    .lines(Paths.get("2021/day01/day01.txt"))
                    .map(Integer::parseInt)
                    .toList();
            
            // index [1,n) -> data[i-1]<data[i] ? 1:0 -> 1 if increase else 0 -> count of increases
            System.out.println(IntStream 
                .range(1, data.size())
                .map((x) -> data.get(x-1) < data.get(x) ? 1 : 0)
                .sum()
            );
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}