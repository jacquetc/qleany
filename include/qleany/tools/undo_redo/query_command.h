// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/qleany_global.h"
#include "undo_redo_command.h"

namespace Qleany::Tools::UndoRedo
{

class QLEANY_EXPORT QueryCommand : public UndoRedoCommand
{
    Q_OBJECT
  public:
    QueryCommand(const QString &text);

    void setQueryFunction(const std::function<Result<void>(QPromise<Result<void>> &promise)> &function);

  private:
    // Overrides the redo() method of UndoRedoCommand
    void redo(QPromise<Result<void>> &progressPromise) override;

    // Overrides the undo() method of UndoRedoCommand
    Result<void> undo() override;

  private:
    std::function<Result<void>(QPromise<Result<void>> &promise)>
        m_queryFunction; /*!< The function to be executed asynchronously when the redo() method is called. */
};
} // namespace Qleany::Tools::UndoRedo
