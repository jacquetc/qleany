// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once
#include "undo_redo_command.h"

namespace Qleany::Tools::UndoRedo {
///
/// \brief The AlterCommand class
/// Used for commands doing actions, in opposition to QueryCommand which for
// read-only requests
template<class Handler, class Request>class AlterCommand : public UndoRedoCommand {
public:

    AlterCommand(const QString& text, Handler *handler, const Request& request)
        : UndoRedoCommand(text), m_handler(handler), m_request(request)
    {
        this->setType(Type::AlterCommand);
        this->setUndoFunction([this]() {
                return Result<void>(m_handler->restore().error());
            });
        this->setRedoFunction([this](QPromise<Result<void> >& progressPromise) {
                m_handler->handle(progressPromise, m_request);
            });
        this->setMergeWithFunction([this](const UndoRedoCommand *other) {
                //            if (other->type() == Type::AlterCommand)
                //            {
                //                const AlterCommand *alterCommand =
                // static_cast<const AlterCommand *>(other);
                //                return
                // m_handler->merge(alterCommand->m_request);
                //            }
                return false;
            });
    }

    // UndoRedoCommand interface

public:

private:

    Handler *m_handler;
    Request m_request;
};
} // namespace Qleany::Tools::UndoRedo
