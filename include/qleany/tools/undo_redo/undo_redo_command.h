// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/common/result.h"
#include "undo_redo_scopes.h"
#include <QFutureWatcher>
#include <QObject>
#include <QPromise>

using namespace Qleany;

namespace Qleany::Tools::UndoRedo
{

class QLEANY_EXPORT UndoRedoCommand : public QObject
{
    Q_OBJECT
  public:
    enum Status
    {
        Waiting,
        Running,
        Finished,
    };
    Q_ENUM(Status)

    UndoRedoCommand(const QString &text);

    virtual Result<void> undo() = 0;

    virtual void redo(QPromise<Result<void>> &progressPromise) = 0;

    void asyncUndo();

    void asyncRedo();

    bool isRunning() const;
    bool isWaiting() const;
    bool isFinished() const;

    Scope scope() const;

    void setScope(Scope newScope);

    QString text() const;

    void setText(const QString &newText);

    bool obsolete() const;

    void setObsolete(bool newObsolete);

    virtual bool mergeWith(const UndoRedoCommand *other);

    bool isSystem() const;
    void setIsSystem(bool newIsSystem);

    QUuid stackId() const;
    void setStackId(const QUuid &newStackId);

  signals:
    void finished(bool isSuccessful);
    /*!
     * \brief A signal that is emitted when a command results in an error.
     * actions.
     */
    void errorSent(Error error);
    void warningSent(Error error);
    void progressFinished();
    void progressRangeChanged(int minimum, int maximum);
    void progressTextChanged(const QString &progressText);
    void progressValueChanged(int progressValue);

    void undoing(Qleany::Tools::UndoRedo::Scope scope, bool active);
    void redoing(Qleany::Tools::UndoRedo::Scope scope, bool active);

  private slots:
    void onFinished();

  private:
    QFutureWatcher<Result<void>> *m_watcher;
    bool m_obsolete; /*!< A boolean representing the obsolete state of the command. */
    bool m_isSystem =
        false;       /*!< A boolean representing the command is a system command (true) or a user command (false). */
    QString m_text;  /*!< A QString representing the text description of the command. */
    Scope m_scope;   /*!< The command's scope as an UndoRedoCommand::Scope enumeration value. */
    Status m_status; /*!< An enum representing the state of the command. */
    QUuid m_stackId; /*!< A QUuid representing the id of the stack the command is in. */
};
} // namespace Qleany::Tools::UndoRedo
