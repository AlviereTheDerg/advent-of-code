
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class day07 {
    public static void main(String[] args) {
        try {
            File input_file = new File("2021/day07/day07.txt");
            Scanner input_scanner = new Scanner(input_file);
            List<Integer> data = new ArrayList<>(Arrays.stream(input_scanner.next().split(",")).map(Integer::parseInt).toList());
            input_scanner.close();
            
            data.sort(null);
            int median = data.get(data.size() / 2);
            int result = data.stream().mapToInt(x -> Math.abs(x - median)).sum();
            System.out.println(result);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
