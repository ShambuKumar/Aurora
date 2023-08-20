#include "defs.h"
#include "move.h"
#include <cstddef>
#include <vector>
namespace Aurora {
class Position {
public:
  std::vector<Move> moves;
  int color[64] = {};
  int piece[64] = {};
  int enemySide;
  int evalScore;
  bool bQCastlingRights;
  bool bKCastlingRights;
  bool wQCastlingRights;
  bool wKCastlingRights;
  void copyPosition(Position &Position) {
    // Copy bool property and side
    enemySide = Position.enemySide;
    bQCastlingRights = Position.bQCastlingRights;
    bKCastlingRights = Position.bKCastlingRights;
    wQCastlingRights = Position.wQCastlingRights;
    // Copy color and piece arrays with loop
    for (int i = 0; i < 64; i++) {
      color[i] = Position.color[i];
      piece[i] = Position.piece[i];
    }
  }
};
}; // namespace Aurora
