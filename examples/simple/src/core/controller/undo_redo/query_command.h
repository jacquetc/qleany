// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_controller_export.h"
#include "undo_redo_command.h"

namespace Simple::Controller::UndoRedo
{

/*!
 * \brief The QueryCommand class
 * \ingroup UndoRedo
 *
 * Invisible command dedicated to queries. QueryCommands will not be stored in UndoRedoSystem as done with
 * UndoRedoCommand, but they will run asynchronously respecting the execution queue of its scope.
 */

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT QueryCommand : public UndoRedoCommand
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
} // namespace Simple::Controller::UndoRedo