// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/qleany_export.h"
#include "undo_redo_command.h"
#include "undo_redo_stack.h"
#include <QEventLoop>
#include <QHash>
#include <QObject>
#include <QQueue>
#include <QSharedPointer>
#include <QTimer>

/*!
    \defgroup UndoRedo
*/
namespace Qleany::Tools::UndoRedo
{
/*!
 * \brief A QObject that manages the undo and redo command history.
 * \ingroup UndoRedo
 */

class QLEANY_EXPORT UndoRedoSystem : public QThread
{
    Q_OBJECT

  public:
    UndoRedoSystem(QObject *parent, Scopes scopes);

    Q_INVOKABLE void run();

    Q_INVOKABLE bool canUndo() const;

    Q_INVOKABLE bool canRedo() const;

    Q_INVOKABLE void undo();

    Q_INVOKABLE void redo();

    Q_INVOKABLE void push(Qleany::Tools::UndoRedo::UndoRedoCommand *command, const QString &commandScope,
                          const QUuid &stackId = QUuid());

    Q_INVOKABLE void push(Qleany::Tools::UndoRedo::UndoRedoCommand *command, const QString &commandScope,
                          const QUuid &stackId = QUuid()) const;

    Q_INVOKABLE void clear();

    Q_INVOKABLE void setUndoLimit(int limit);

    Q_INVOKABLE int undoLimit() const;

    Q_INVOKABLE QString undoText() const;

    Q_INVOKABLE QString redoText() const;

    Q_INVOKABLE QStringList undoRedoTextList() const;

    Q_INVOKABLE int currentIndex() const;

    Q_INVOKABLE void setCurrentIndex(int index);

    Q_INVOKABLE void setActiveStack(const QUuid &stackId = QUuid());

    Q_INVOKABLE QUuid activeStackId() const;

    Q_INVOKABLE QStringList queuedCommandTextListByScope(const QString &scopeFlagString) const;

    Q_INVOKABLE bool isRunning() const;

    Q_INVOKABLE int numberOfCommands() const;

  private Q_SLOTS:

    void executeNextCommandUndo();
    void executeNextCommandRedo();
    void onCommandDoFinished(bool isSuccessful);

    void onCommandUndoFinished(bool isSuccessful);
    void onCommandRedoFinished(bool isSuccessful);

  Q_SIGNALS:

    void stateChanged();

    void warningSent(Error error);
    void errorSent(Error error);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     * start redoing.
     * actions.
     */
    void redoing(Scope scope, bool active);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     * start undoing.
     * actions.
     */
    void undoing(Scope scope, bool active);

  private:
    void executeNextCommandDo(const ScopeFlag &scopeFlag, const QUuid &stackId);
    bool isCommandAllowedToRun(QSharedPointer<UndoRedoCommand> command, const ScopeFlag &currentScopeFlag);
    bool areQueuesEmpty() const;

    int m_undoLimit; /*!< The maximum number of undo commands that can be stored
                        in the undo-redo system. */
    Scopes m_scopes;
    QSharedPointer<UndoRedoStack> m_activeStack;
    QHash<QUuid, QSharedPointer<UndoRedoStack>> m_stackHash;
    QQueue<QSharedPointer<UndoRedoCommand>> m_setIndexCommandQueue;
    bool m_readyAfterSettingIndex = true;
    QHash<ScopeFlag, QQueue<QSharedPointer<UndoRedoCommand>>> m_scopedCommandQueueHash;
    QHash<ScopeFlag, QSharedPointer<UndoRedoCommand>> m_currentCommandHash;
};
} // namespace Qleany::Tools::UndoRedo
