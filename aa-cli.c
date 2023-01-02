#include <time.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#define max(a, b)           \
  ({                        \
    __typeof__(a) _a = (a); \
    __typeof__(b) _b = (b); \
    _a > _b ? _a : _b;      \
  })

#define min(a, b)           \
  ({                        \
    __typeof__(a) _a = (a); \
    __typeof__(b) _b = (b); \
    _a < _b ? _a : _b;      \
  })

// Set up unit types
typedef struct {
  char type[32];
  int ipc;
  int attack;
  int defend;
} Unit;

Unit Infantry = {"Infantry", 3, 1, 2};
Unit Tank = {"Tank", 6, 3, 3};
Unit Fighter = {"Fighter", 10, 3, 4};
Unit Bomber = {"Bomber", 10, 4, 1};
Unit Battleship = {"Battleship", 10, 4, 4};

// Generate number in range [min,max)
int rand_between(int min, int max){
    int number = min + rand() % (max - min);
    return number;
}

Unit **build_unit_list(char *input, int length) {
  Unit ** result = malloc(length * sizeof(Unit*));

  for (int i = 0; i < length; i++)
  {
    switch (input[i])
    {
    case 'I':
      result[i] = &Infantry;
      break;
    case 'T':
      result[i] = &Tank;
      break;
    case 'F':
      result[i] = &Fighter;
      break;
    case 'b':
      result[i] = &Bomber;
      break;
    case 'B':
      result[i] = &Battleship;
      break;
    }
  }

  return result;
}

// Run the program
int main(int argc, char *argv[]) {
  // Check argument count
  if (argc < 4) {
    printf("Provide inputs for both attackers and defenders and sim count.\n");
    return 1;
  }

  // Initialize randomization
  srand(time(NULL));
  clock_t start, end;
  double time_elapsed;
  start = clock();

  int num_attackers = strlen(argv[1]);
  int num_defenders = strlen(argv[2]);
  int sim_count = atoi(argv[3]);

  // Variable-sized arrays based on input
  Unit **attacker_units = build_unit_list(argv[1], num_attackers);
  Unit **defender_units = build_unit_list(argv[2], num_defenders);

  // DEBUG:
  // for (int i=0; i<num_attackers; i++) {
  //   Unit unit = *attacker_units[i];
  //   printf("Attacker Unit:\n");
  //   printf("    Type: %s\n", unit.type);
  //   printf("    IPC: %i\n", unit.ipc);
  //   printf("    Attack: %i\n", unit.attack);
  //   printf("    Defend: %i\n", unit.defend);
  // }

  // printf("\n");
  // for (int i=0; i<num_defenders; i++) {
  //   Unit unit = *defender_units[i];
  //   printf("Defender Unit:\n");
  //   printf("    Type: %s\n", unit.type);
  //   printf("    IPC: %i\n", unit.ipc);
  //   printf("    Attack: %i\n", unit.attack);
  //   printf("    Defend: %i\n", unit.defend);
  // }
  // END DEBUG

  // TODO: Sort arrays by IPC count

  int a_wins = 0;
  int d_wins = 0;
  int draws = 0;
  for (int i=0; i<sim_count; i++) {
    // Run combat one time. Casualties will be taken in
    // order of the attacker and defender lists.
    int done = 0;
    int a_casualties = 0;
    int d_casualties = 0;
    while (!done)
    {
      // Roll for each attacking unit, and
      // tally new defender casualties -- these
      // are not applied until the end of the round.
      int new_d_casualties = 0;
      for (int i = a_casualties; i < num_attackers; i++)
      {
        int roll = rand_between(1, 7);
        Unit unit = *attacker_units[i];
        if (roll <= unit.attack)
        {
          new_d_casualties++;
        }
      }

      // Roll for each defending unit and
      // immediately take attacker casualties
      int new_a_casualties = 0;
      for (int i = d_casualties; i < num_defenders; i++)
      {
        int roll = rand_between(1, 7);
        Unit unit = *defender_units[i];
        if (roll <= unit.defend)
        {
          new_a_casualties++;
        }
      }

      // DEBUG Lines
      // printf("Attacker got %i hit/s\n", new_d_casualties);
      // printf("Defender got %i hit/s\n\n", new_a_casualties);

      // Don't take more casualties than number of units
      a_casualties = min(a_casualties += new_a_casualties, num_attackers);
      d_casualties = min(d_casualties += new_d_casualties, num_defenders);

      if (a_casualties == num_attackers || d_casualties == num_defenders)
      {
        done = 1;
      }
    }

    // SUMMARY OF A SINGLE COMBAT
    // printf("\n--SUMMARY--\n");
    // printf("Attacker hit %i of %i\n", d_casualties, num_defenders);
    // printf("Defender hit %i of %i\n", a_casualties, num_attackers);

    if (a_casualties == num_attackers && d_casualties == num_defenders) {
      draws++;
      // printf("It's a draw.\n");
    } else if (a_casualties == num_attackers) {
      d_wins++;
      // printf("Defender wins.\n");
    } else if (d_casualties == num_defenders) {
      a_wins++;
      // printf("Attacker wins.\n");
    }
  }

  end = clock();
  time_elapsed = ((double)(end - start)) / CLOCKS_PER_SEC;

  // SUMMARY OF SIMULATION
  printf("\n--SUMMARY--\n");
  printf("Sim Duration: %f ms\n", time_elapsed * 1000);
  printf("Attacker wins: %i\n", a_wins);
  printf("Defender wins: %i\n", d_wins);
  printf("Draws: %i\n", draws);
  printf("Success Chance: %f \n", (float)a_wins/(float)sim_count);

  return 0;
}