
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.stream.IntStream;

public class day04 {
    private static class BingoCard {
        boolean[] board_calls;
        Map<Integer, Integer> board_positions;
        int last_call;

        public BingoCard(String card_data) {
            board_calls = new boolean[25]; // initializes to false
            board_positions = new HashMap<>();
            String[] array_card_data = card_data.trim().split("\\s+");
            for (int index = 0; index < array_card_data.length; index++) {
                board_positions.put(Integer.parseInt(array_card_data[index]), index);
            }
            last_call = 0;
        }

        public boolean draw(int call) {
            if (board_positions.containsKey(call)) {
                board_calls[board_positions.get(call)] = true;
            }
            last_call = call;
            return this.solved();
        }

        public boolean solved() {
            boolean rows = IntStream.range(0,5).boxed().anyMatch((r) -> {
                return IntStream.range(0,5).map(c -> 5*r+c).boxed().allMatch(i -> board_calls[i]);
            });
            boolean columns = IntStream.range(0,5).boxed().anyMatch((r) -> {
                return IntStream.range(0,5).map(c -> r+5*c).boxed().allMatch(i -> board_calls[i]);
            });
            return rows || columns;
        }

        public int score() {
            return board_positions.entrySet().stream().filter(i -> !board_calls[i.getValue()]).mapToInt(i -> i.getKey()).sum() * last_call;
        }
    }
    public static void main(String[] args) {
        try {
            File input_file = new File("2021/day04/day04.txt");
            Scanner input_scanner = new Scanner(input_file);
            input_scanner.useDelimiter("\n\n");

            List<Integer> calls = Arrays.stream(input_scanner.next().split(",")).map(Integer::parseInt).toList();

            List<BingoCard> cards = new ArrayList<>();
            while (input_scanner.hasNext())
                cards.add(new BingoCard(input_scanner.next()));
            input_scanner.close();

            BingoCard selected = null;
            boolean escape = false;
            for (int call : calls) {
                for (BingoCard card : cards) {
                    if (card.draw(call)) {
                        escape = true;
                        selected = card;
                    }
                    if (escape) break;
                }
                if (escape) break;
            }
            System.out.println(selected != null ? selected.score() : "null");
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
