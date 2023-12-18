// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#include "qleany/tools/undo_redo/query_command.h"

using namespace Qleany::Tools::UndoRedo;

/*!
 * \brief Constructs a QueryCommand instance with the specified text for logging purposes.
 * \param text A QString representing the message for logging purposes.
 */
QueryCommand::QueryCommand(const QString &text) : UndoRedoCommand(text)
{
    this->setType(Type::QueryCommand);
}

/*!
 * \brief Sets the function to be executed asynchronously when "redo()" is called in UndoRedoSystem.
 * \param function A std::function<void()> representing the function to be executed.
 */
void QueryCommand::setQueryFunction(const std::function<Result<void>(QPromise<Result<void>> &promise)> &function)
{
    m_queryFunction = function;
    this->setRedoFunction(
        [this](QPromise<Result<void>> &progressPromise) { return Result<void>(m_queryFunction(progressPromise)); });
}
