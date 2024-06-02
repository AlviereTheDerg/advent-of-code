
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

            int average = (int) Math.round(data.stream().mapToDouble(x -> x).sum() / data.size());
            long pt2_0 = data.stream().mapToLong(x -> Math.abs(x - average)).map(x -> x*(x+1) / 2).sum();
            long pt2_n1 = data.stream().mapToLong(x -> Math.abs(x - average + 1)).map(x -> x*(x+1) / 2).sum();
            long pt2_p1 = data.stream().mapToLong(x -> Math.abs(x - average - 1)).map(x -> x*(x+1) / 2).sum();
            long pt2_result = Math.min(pt2_0, Math.min(pt2_n1, pt2_p1));
            System.out.println(pt2_result);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
