// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/qleany_export.h"
#include "undo_redo_command.h"

namespace Qleany::Tools::UndoRedo
{

/*!
 * \brief The QueryCommand class
 * \ingroup UndoRedo
 *
 * Invisible command dedicated to queries. QueryCommands will not be stored in UndoRedoSystem as done with
 * UndoRedoCommand, but they will run asynchronously respecting the execution queue of its scope.
 */

class QLEANY_EXPORT QueryCommand : public UndoRedoCommand
{
    Q_OBJECT
  public:
    QueryCommand(const QString &text);

    void setQueryFunction(const std::function<Result<void>(QPromise<Result<void>> &promise)> &function);

  private:
  private:
    std::function<Result<void>(QPromise<Result<void>> &promise)>
        m_queryFunction; /*!< The function to be executed asynchronously when the redo() method is called. */
};
} // namespace Qleany::Tools::UndoRedo
