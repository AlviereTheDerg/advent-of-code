
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

public class day08 {
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
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}