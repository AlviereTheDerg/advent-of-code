
import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class day24 {
    private static class MONADStage {
        int A, B, C;

        public MONADStage(List<String> raw_input) {
            // only 3 values differ between each 'stage' of the process
            // simplifies down to:
            /*
             * w = input
             * if (w == z%26 + B)
             *   z /= A
             * else
             *   z /= A
             *   z *= 26
             *   z += w + C
             */
            A = Integer.parseInt(raw_input.get(4).substring(6));
            B = Integer.parseInt(raw_input.get(5).substring(6));
            C = Integer.parseInt(raw_input.get(15).substring(6));
        }

        public int process(int z, int w) {
            int x = (w == (z % 26) + B) ? 0 : 1;
            z /= A;
            z *= (25 * x) + 1;
            z += (w + C) * x;
            return z;
        }
    }

    public static Map<Integer, Integer> extract_stack_pairings(List<MONADStage> stages) {
        // z is a stack
        // if A==1 OR w check fails: push w+C to stack
        // otherwise pop
        // half of MONADStages have A==1, thus w check MUST PASS WHEN ABLE TO
        // w check: head of stack == w-B
        // goal: z is empty

        // identify pairs of 'stage number where value pushed' vs 'stage number where value popped'
        HashMap<Integer, Integer> pairings = new HashMap<>();
        LinkedList<Integer> identification_stack = new LinkedList<>();
        for (int i = 0; i < stages.size(); i++) {
            if (stages.get(i).A == 1) {
                identification_stack.push(i);
            } else {
                pairings.put(identification_stack.pop(), i);
            }
        }
        return pairings;
    }

    public static long get_max_model_number(List<MONADStage> stages, Map<Integer, Integer> pairings) {
        // construct max model number from the pairs and their offsets
        int[] max_model = new int[stages.size()];
        for (Map.Entry<Integer,Integer> entry : pairings.entrySet()) {
            int difference = stages.get(entry.getKey()).C + stages.get(entry.getValue()).B;
            if (difference > 0) {
                max_model[entry.getKey()] = 9 - difference;
                max_model[entry.getValue()] = 9;
            } else {
                max_model[entry.getKey()] = 9;
                max_model[entry.getValue()] = 9 + difference;
            }
        }
        long z = 0;
        for (int i = 0; i < stages.size(); i++)
            z = 10*z + max_model[i];
        return z;
    }

    public static long get_min_model_number(List<MONADStage> stages, Map<Integer, Integer> pairings) {
        // construct min model number from the pairs and their offsets
        int[] max_model = new int[stages.size()];
        for (Map.Entry<Integer,Integer> entry : pairings.entrySet()) {
            int difference = stages.get(entry.getKey()).C + stages.get(entry.getValue()).B;
            if (difference > 0) {
                max_model[entry.getKey()] = 1;
                max_model[entry.getValue()] = 1 + difference;
            } else {
                max_model[entry.getKey()] = 1 - difference;
                max_model[entry.getValue()] = 1;
            }
        }
        long z = 0;
        for (int i = 0; i < stages.size(); i++)
            z = 10*z + max_model[i];
        return z;
    }

    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day24/day24.txt"));
            List<String> input_buffer = new ArrayList<>();
            List<MONADStage> stages = new ArrayList<>();
            while (input_scanner.hasNext()) {
                String buffer = input_scanner.nextLine();
                if (input_buffer.size() > 0 && buffer.equals("inp w")) {
                    stages.add(new MONADStage(input_buffer));
                    input_buffer.clear();
                }
                input_buffer.add(buffer);
            }
            stages.add(new MONADStage(input_buffer));
            input_scanner.close();

            Map<Integer, Integer> pairings = extract_stack_pairings(stages);
            System.out.println(get_max_model_number(stages, pairings));
            System.out.println(get_min_model_number(stages, pairings));

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
